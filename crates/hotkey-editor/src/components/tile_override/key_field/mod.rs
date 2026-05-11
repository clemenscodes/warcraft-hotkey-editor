use dioxus::prelude::*;

/// A single key-capture button in the override panel header.
/// Clicking it activates hotkey editing mode for the associated field.
#[derive(Props, Clone, PartialEq)]
pub(crate) struct OverrideKeyFieldProps {
    /// The visible label — a single letter, "–", "Esc", "Mouse4", etc.
    pub(crate) label: String,
    /// Whether this cell is currently in capture/editing state.
    pub(crate) is_editing: bool,
    /// Whether the token is a non-letter special token (Esc, Mouse4, Mouse5).
    /// Controls `data-special` so CSS can widen the cell for multi-char labels.
    pub(crate) is_special: bool,
    /// Human-readable title for the button (used by tooltip / accessibility).
    pub(crate) title: String,
    /// Called when the player clicks to start editing.
    pub(crate) on_activate: EventHandler<()>,
}

#[component]
pub(crate) fn OverrideKeyField(props: OverrideKeyFieldProps) -> Element {
    let label = props.label;
    let is_editing = props.is_editing;
    let is_special = props.is_special;
    let title = props.title;
    let on_activate = props.on_activate;
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
            {label}
        }
    }
}
