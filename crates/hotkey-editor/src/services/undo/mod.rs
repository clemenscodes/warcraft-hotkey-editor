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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum UndoDirection {
    Undo,
    Redo,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct KeyboardUndoRequest {
    generation: u32,
    direction: UndoDirection,
}

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
        self.history.peek().clone()
    }

    fn replace(&self, aggregate: EditorHistory) {
        let mut history_signal = self.history;
        history_signal.set(aggregate);
    }

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

impl UndoHistory {
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
