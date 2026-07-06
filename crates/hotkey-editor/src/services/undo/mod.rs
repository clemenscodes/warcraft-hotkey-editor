pub mod context;

use crate::repository::editor_history_repository::EditorHistoryRepository;
use ddd::ApplicationLayer;
use ddd::ApplicationService;
use ddd::Layered;
use ddd::Repository;
use ddd::Service;
use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::EditorHistory;
use warcraft_keybinds::EditorSnapshot;
use warcraft_keybinds::GridLayout;

/// Which direction a keyboard shortcut requested. Constructed only by the
/// wasm-only keyboard listener; the native build reads but never builds it.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum UndoDirection {
    Undo,
    Redo,
}

/// A keyboard-shortcut request, carrying a monotonically increasing generation so
/// each keypress is a distinct value (even repeats of the same direction). The
/// window keydown listener only *sets* this signal; a reactive effect performs the
/// undo/redo, where signal reads are valid.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct KeyboardUndoRequest {
    generation: u32,
    direction: UndoDirection,
}

/// The application-layer undo/redo service. It owns the live [`EditorHistory`]
/// aggregate as a signal and is a [`Service`] over it; the pure timeline lives in
/// the domain, persistence and compression live in the infrastructure layers, and
/// this type keeps only the renderer glue: applying a restored snapshot back to the
/// live keys/grid signals, the debounced persist, and the keyboard listener.
#[derive(Clone, Copy)]
pub struct UndoHistory {
    keys: Signal<Option<CustomKeys>>,
    grid_layout: Signal<GridLayout>,
    history: Signal<EditorHistory>,
    persist_generation: Signal<u32>,
    keyboard_request: Signal<Option<KeyboardUndoRequest>>,
    handled_request_generation: Signal<u32>,
}

impl UndoHistory {
    /// Custom hook: loads any persisted timeline, reseats its present on the actual
    /// boot state (so the first capture-effect run is a no-op rather than a spurious
    /// entry), and creates the history and keyboard signals.
    pub fn use_history(keys: Signal<Option<CustomKeys>>, grid_layout: Signal<GridLayout>) -> Self {
        let boot_present = snapshot_from_state(&keys, &grid_layout);
        let repository = EditorHistoryRepository;
        let mut loaded_history = repository.load().unwrap_or_default();
        loaded_history.reseat_present(boot_present);
        let history = use_signal(|| loaded_history);
        let persist_generation = use_signal(|| 0_u32);
        let keyboard_request = use_signal(|| None);
        let handled_request_generation = use_signal(|| 0_u32);
        Self {
            keys,
            grid_layout,
            history,
            persist_generation,
            keyboard_request,
            handled_request_generation,
        }
    }

    /// Performs the latest pending keyboard request, if any is unhandled. Meant to
    /// be driven from a reactive effect (it reads the request signal and the
    /// stacks); the window listener only writes `keyboard_request`.
    pub(crate) fn handle_keyboard_request(&self) {
        let Some(request) = *self.keyboard_request.read() else {
            return;
        };
        if request.generation == *self.handled_request_generation.peek() {
            return;
        }
        let mut handled_request_generation = self.handled_request_generation;
        handled_request_generation.set(request.generation);
        match request.direction {
            UndoDirection::Undo => self.undo(),
            UndoDirection::Redo => self.redo(),
        }
    }

    pub(crate) fn can_undo(&self) -> bool {
        self.history.read().can_undo()
    }

    pub(crate) fn can_redo(&self) -> bool {
        self.history.read().can_redo()
    }

    /// Records a transition to `current`. A no-op when `current` equals the present
    /// state (so restores never create new history) — and crucially, a no-op records
    /// nothing *and persists nothing*. The boot capture effect fires one such no-op
    /// on first render; were it to schedule a persist, a fresh visit that never
    /// touched anything would still write an empty-stack blob after the debounce,
    /// which a later reload would restore as "nothing to undo".
    pub(crate) fn record(&self, current: EditorSnapshot) {
        let mut aggregate = self.snapshot();
        let recorded = aggregate.record(current);
        if !recorded {
            return;
        }
        self.replace(aggregate);
        self.schedule_persist();
    }

    pub(crate) fn undo(&self) {
        let restored = self.commit(EditorHistory::undo);
        if let Some(snapshot) = restored {
            self.apply(&snapshot);
        }
    }

    pub(crate) fn redo(&self) {
        let restored = self.commit(EditorHistory::redo);
        if let Some(snapshot) = restored {
            self.apply(&snapshot);
        }
    }

    fn apply(&self, snapshot: &EditorSnapshot) {
        let mut keys = self.keys;
        let mut grid_layout = self.grid_layout;
        let restored_keys = CustomKeys::from_text(snapshot.keys_text());
        keys.set(Some(restored_keys));
        if let Ok(restored_layout) = GridLayout::try_from(snapshot.grid_layout_text()) {
            grid_layout.set(restored_layout);
        }
    }

    /// Debounced persistence: the compressed blob is only written ~1s after the
    /// last change, so a burst of actions doesn't pay the compression cost each
    /// time. A generation counter cancels superseded timers.
    fn schedule_persist(&self) {
        let mut persist_generation = self.persist_generation;
        let next_generation = persist_generation.peek().wrapping_add(1);
        persist_generation.set(next_generation);
        let history = *self;
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(1000).await;
            if *history.persist_generation.peek() == next_generation {
                history.persist();
            }
        });
    }

    fn persist(&self) {
        let aggregate = self.snapshot();
        let repository = self.repository();
        repository.save(&aggregate);
    }
}

impl Layered for UndoHistory {
    type Layer = ApplicationLayer;
}

impl ApplicationService for UndoHistory {}

impl Service<EditorHistory> for UndoHistory {
    type Repository = EditorHistoryRepository;

    fn repository(&self) -> Self::Repository {
        EditorHistoryRepository
    }

    fn snapshot(&self) -> EditorHistory {
        // `peek`, not `read`: `commit` calls `snapshot` and then writes the
        // `history` signal via `replace`. `record` runs inside the shell's
        // reactive capture effect, so a subscribing `read` here would make that
        // effect depend on `history` and re-fire on its own write — an infinite
        // render loop. A snapshot is a point-in-time clone to mutate, never a
        // subscription.
        self.history.peek().clone()
    }

    fn replace(&self, aggregate: EditorHistory) {
        let mut history_signal = self.history;
        history_signal.set(aggregate);
    }

    /// Overridden to preserve the debounced write-through: the aggregate is
    /// updated and the live signal replaced immediately, but the compressed blob is
    /// persisted on a 1-second debounce (through the repository) rather than on
    /// every keystroke.
    fn commit<Outcome>(&self, change: impl FnOnce(&mut EditorHistory) -> Outcome) -> Outcome {
        let mut aggregate = self.snapshot();
        let outcome = change(&mut aggregate);
        self.replace(aggregate);
        self.schedule_persist();
        outcome
    }
}

fn snapshot_from_state(
    keys: &Signal<Option<CustomKeys>>,
    grid_layout: &Signal<GridLayout>,
) -> EditorSnapshot {
    let keys_text = keys
        .peek()
        .as_ref()
        .map(|file| file.normalize().to_string())
        .unwrap_or_default();
    let grid_layout_text = grid_layout.peek().to_storage_string();
    EditorSnapshot::new(keys_text, grid_layout_text)
}

#[cfg(target_arch = "wasm32")]
impl UndoHistory {
    /// Installs a window-level keydown listener for Ctrl/Cmd+Z (undo) and
    /// Ctrl/Cmd+Shift+Z or Ctrl/Cmd+Y (redo). Suppressed while focus is in a text
    /// field so the browser's native text undo keeps working there. The closure is
    /// leaked for the page lifetime (one-time install at boot).
    pub(crate) fn install_keyboard_shortcuts(self) {
        use std::cell::Cell;
        use std::rc::Rc;
        use wasm_bindgen::JsCast;
        use wasm_bindgen::closure::Closure;
        let Some(window) = web_sys::window() else {
            return;
        };
        let request_generation = Rc::new(Cell::new(0_u32));
        let mut keyboard_request = self.keyboard_request;
        let closure = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
            move |event: web_sys::KeyboardEvent| {
                let key = event.key();
                let is_z = key.eq_ignore_ascii_case("z");
                let is_y = key.eq_ignore_ascii_case("y");
                if !is_z && !is_y {
                    return;
                }
                if !(event.ctrl_key() || event.meta_key()) {
                    return;
                }
                if editable_target_is_focused() {
                    return;
                }
                event.prevent_default();
                let redo_requested = is_y || (is_z && event.shift_key());
                let direction = if redo_requested {
                    UndoDirection::Redo
                } else {
                    UndoDirection::Undo
                };
                let next_generation = request_generation.get().wrapping_add(1);
                request_generation.set(next_generation);
                let request = KeyboardUndoRequest {
                    generation: next_generation,
                    direction,
                };
                keyboard_request.set(Some(request));
            },
        );
        let listener = closure.as_ref().unchecked_ref();
        let _ = window.add_event_listener_with_callback("keydown", listener);
        closure.forget();
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl UndoHistory {
    pub(crate) fn install_keyboard_shortcuts(self) {}
}

#[cfg(target_arch = "wasm32")]
fn editable_target_is_focused() -> bool {
    use wasm_bindgen::JsCast;
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return false;
    };
    let Some(active_element) = document.active_element() else {
        return false;
    };
    let tag_name = active_element.tag_name().to_ascii_lowercase();
    if tag_name == "input" || tag_name == "textarea" || tag_name == "select" {
        return true;
    }
    active_element
        .dyn_ref::<web_sys::HtmlElement>()
        .is_some_and(web_sys::HtmlElement::is_content_editable)
}
