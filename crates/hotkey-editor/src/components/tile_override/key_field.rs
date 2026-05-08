use dioxus::prelude::*;

/// A single key-capture button in the override panel header.
/// Clicking it activates hotkey editing mode for the associated field.
#[component]
pub(crate) fn OverrideKeyField(
    /// The visible label — a single letter, "–", "Esc", "Mouse4", etc.
    label: String,
    /// Whether this cell is currently in capture/editing state.
    is_editing: bool,
    /// Whether the token is a non-letter special token (Esc, Mouse4, Mouse5).
    /// Controls `data-special` so CSS can widen the cell for multi-char labels.
    is_special: bool,
    /// Human-readable title for the button (used by tooltip / accessibility).
    title: String,
    /// Called when the player clicks to start editing.
    on_activate: EventHandler<()>,
) -> Element {
    let cell_class = if is_editing {
        "override-key-cell editing"
    } else {
        "override-key-cell"
    };
    let special_flag = if is_special { "true" } else { "false" };
    let handle_click = move |_| on_activate.call(());
    rsx! {
        button {
            class: cell_class,
            "data-special": special_flag,
            title: title,
            onclick: handle_click,
            "{label}"
        }
    }
}
