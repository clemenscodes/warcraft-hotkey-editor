use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

use crate::model::grid::GridLayout;
use crate::services::storage::local_storage::LocalStorage;

const UNDO_STORAGE: LocalStorage = LocalStorage::new("warcraft-hotkey-editor.undo-history");

/// Maximum number of snapshots kept per stack. Each snapshot is the full
/// canonical state; the on-disk blob is deflate-compressed (the materialized
/// text is highly repetitive), so a deep history still fits localStorage.
const MAX_DEPTH: usize = 40;

/// Separators chosen from the ASCII control range so they can never appear in
/// the INI-style CustomKeys text or the grid-layout storage string.
const FIELD_SEPARATOR: char = '\u{1f}';
const RECORD_SEPARATOR: char = '\u{1e}';
const GROUP_SEPARATOR: char = '\u{1d}';

/// One complete, restorable editor state: the canonical keys text plus the grid
/// layout. Because localStorage already holds the *entire* normalized state as a
/// single string, a snapshot of that string (plus the layout) is the whole app
/// state — so every action, large or small, is captured uniformly.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct EditorSnapshot {
    keys_text: String,
    grid_layout_text: String,
}

impl EditorSnapshot {
    pub(crate) fn new(keys_text: String, grid_layout_text: String) -> Self {
        Self {
            keys_text,
            grid_layout_text,
        }
    }

    fn encode(&self) -> String {
        format!(
            "{}{FIELD_SEPARATOR}{}",
            self.keys_text, self.grid_layout_text
        )
    }

    fn decode(encoded: &str) -> Option<Self> {
        let mut fields = encoded.splitn(2, FIELD_SEPARATOR);
        let keys_text = fields.next()?.to_owned();
        let grid_layout_text = fields.next()?.to_owned();
        Some(Self {
            keys_text,
            grid_layout_text,
        })
    }
}

/// Which direction a keyboard shortcut requested. Constructed only by the
/// wasm-only keyboard listener; the native build reads but never builds it.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum UndoDirection {
    Undo,
    Redo,
}

/// A keyboard-shortcut request, carrying a monotonically increasing generation
/// so each keypress is a distinct value (even repeats of the same direction).
/// The window keydown listener only *sets* this signal — it never reads a signal
/// — because signal reads from outside the Dioxus runtime return stale values.
/// A reactive effect then performs the undo/redo, where reads are valid.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct KeyboardUndoRequest {
    generation: u32,
    direction: UndoDirection,
}

/// A single global undo/redo timeline backed by full-state snapshots. Every
/// committed mutation funnels through the two state signals, so a capture effect
/// records one snapshot per action; undo/redo restore a snapshot by writing it
/// back to those signals (which re-persists through the normal storage effects).
#[derive(Clone, Copy)]
pub struct UndoHistory {
    keys: Signal<Option<CustomKeys>>,
    grid_layout: Signal<GridLayout>,
    undo_stack: Signal<Vec<EditorSnapshot>>,
    redo_stack: Signal<Vec<EditorSnapshot>>,
    present: Signal<EditorSnapshot>,
    persist_generation: Signal<u32>,
    keyboard_request: Signal<Option<KeyboardUndoRequest>>,
    handled_request_generation: Signal<u32>,
}

impl UndoHistory {
    /// Custom hook: creates the history signals (restoring any persisted stacks)
    /// seeded with the current boot state as `present`, so the first capture-
    /// effect run is a no-op rather than a spurious entry.
    pub fn use_history(keys: Signal<Option<CustomKeys>>, grid_layout: Signal<GridLayout>) -> Self {
        let boot_snapshot = snapshot_from_state(&keys, &grid_layout);
        let persisted_stacks = load_persisted_stacks();
        let undo_entries = persisted_stacks.undo_entries;
        let redo_entries = persisted_stacks.redo_entries;
        let undo_stack = use_signal(|| undo_entries);
        let redo_stack = use_signal(|| redo_entries);
        let present = use_signal(|| boot_snapshot);
        let persist_generation = use_signal(|| 0_u32);
        let keyboard_request = use_signal(|| None);
        let handled_request_generation = use_signal(|| 0_u32);
        Self {
            keys,
            grid_layout,
            undo_stack,
            redo_stack,
            present,
            persist_generation,
            keyboard_request,
            handled_request_generation,
        }
    }

    /// Performs the latest pending keyboard request, if any is unhandled. Meant
    /// to be driven from a reactive effect (it reads the request signal and the
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
        !self.undo_stack.read().is_empty()
    }

    pub(crate) fn can_redo(&self) -> bool {
        !self.redo_stack.read().is_empty()
    }

    /// Records a transition to `current`. A no-op when `current` equals the
    /// present state — which is exactly what happens right after undo/redo
    /// restores a snapshot, so restores never create new history entries.
    pub(crate) fn record(&self, current: EditorSnapshot) {
        if current == *self.present.peek() {
            return;
        }
        let mut undo_stack = self.undo_stack;
        let mut redo_stack = self.redo_stack;
        let mut present = self.present;
        let previous = present.peek().clone();
        {
            let mut stack_guard = undo_stack.write();
            stack_guard.push(previous);
            while stack_guard.len() > MAX_DEPTH {
                stack_guard.remove(0);
            }
        }
        redo_stack.write().clear();
        present.set(current);
        self.schedule_persist();
    }

    pub(crate) fn undo(&self) {
        let mut undo_stack = self.undo_stack;
        if undo_stack.peek().is_empty() {
            return;
        }
        let mut redo_stack = self.redo_stack;
        let mut present = self.present;
        let restored = undo_stack
            .write()
            .pop()
            .expect("undo stack is non-empty here");
        let current = present.peek().clone();
        redo_stack.write().push(current);
        present.set(restored.clone());
        self.apply(&restored);
        self.schedule_persist();
    }

    pub(crate) fn redo(&self) {
        let mut redo_stack = self.redo_stack;
        if redo_stack.peek().is_empty() {
            return;
        }
        let mut undo_stack = self.undo_stack;
        let mut present = self.present;
        let restored = redo_stack
            .write()
            .pop()
            .expect("redo stack is non-empty here");
        let current = present.peek().clone();
        undo_stack.write().push(current);
        present.set(restored.clone());
        self.apply(&restored);
        self.schedule_persist();
    }

    fn apply(&self, snapshot: &EditorSnapshot) {
        let mut keys = self.keys;
        let mut grid_layout = self.grid_layout;
        let restored_keys = CustomKeys::from(snapshot.keys_text.as_str());
        keys.set(Some(restored_keys));
        if let Ok(restored_layout) = GridLayout::try_from(snapshot.grid_layout_text.as_str()) {
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
        let undo_guard = self.undo_stack.peek();
        let redo_guard = self.redo_stack.peek();
        let serialized = serialize_stacks(&undo_guard, &redo_guard);
        let compressed = compress_blob(&serialized);
        UNDO_STORAGE.set(&compressed);
    }
}

struct PersistedStacks {
    undo_entries: Vec<EditorSnapshot>,
    redo_entries: Vec<EditorSnapshot>,
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

fn serialize_stacks(undo: &[EditorSnapshot], redo: &[EditorSnapshot]) -> String {
    let record_separator = RECORD_SEPARATOR.to_string();
    let encode_stack = |stack: &[EditorSnapshot]| {
        let encoded: Vec<String> = stack.iter().map(EditorSnapshot::encode).collect();
        encoded.join(&record_separator)
    };
    let undo_text = encode_stack(undo);
    let redo_text = encode_stack(redo);
    format!("{undo_text}{GROUP_SEPARATOR}{redo_text}")
}

fn load_persisted_stacks() -> PersistedStacks {
    let empty = PersistedStacks {
        undo_entries: Vec::new(),
        redo_entries: Vec::new(),
    };
    let Some(raw) = UNDO_STORAGE.get() else {
        return empty;
    };
    let Some(serialized) = decompress_blob(&raw) else {
        return empty;
    };
    let mut groups = serialized.splitn(2, GROUP_SEPARATOR);
    let undo_part = groups.next().unwrap_or_default();
    let redo_part = groups.next().unwrap_or_default();
    PersistedStacks {
        undo_entries: parse_stack(undo_part),
        redo_entries: parse_stack(redo_part),
    }
}

fn parse_stack(part: &str) -> Vec<EditorSnapshot> {
    if part.is_empty() {
        return Vec::new();
    }
    part.split(RECORD_SEPARATOR)
        .filter_map(EditorSnapshot::decode)
        .collect()
}

fn compress_blob(text: &str) -> String {
    use base64::Engine;
    use flate2::Compression;
    use flate2::write::DeflateEncoder;
    use std::io::Write;

    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    let _ = encoder.write_all(text.as_bytes());
    let compressed_bytes = encoder.finish().unwrap_or_default();
    base64::engine::general_purpose::STANDARD.encode(compressed_bytes)
}

fn decompress_blob(encoded: &str) -> Option<String> {
    use base64::Engine;
    use flate2::read::DeflateDecoder;
    use std::io::Read;

    let compressed_bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .ok()?;
    let mut decoder = DeflateDecoder::new(compressed_bytes.as_slice());
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed).ok()?;
    Some(decompressed)
}

#[cfg(target_arch = "wasm32")]
impl UndoHistory {
    /// Installs a window-level keydown listener for Ctrl/Cmd+Z (undo) and
    /// Ctrl/Cmd+Shift+Z or Ctrl/Cmd+Y (redo). Suppressed while focus is in a
    /// text field so the browser's native text undo keeps working there. The
    /// closure is leaked for the page lifetime (one-time install at boot).
    pub(crate) fn install_keyboard_shortcuts(self) {
        use std::cell::Cell;
        use std::rc::Rc;

        use wasm_bindgen::JsCast;
        use wasm_bindgen::closure::Closure;

        let Some(window) = web_sys::window() else {
            return;
        };
        // Plain (non-signal) counter owned by the closure, so the listener never
        // reads a Dioxus signal — only writes `keyboard_request`. The effect
        // wired in the app then runs the actual undo/redo in a reactive context.
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
