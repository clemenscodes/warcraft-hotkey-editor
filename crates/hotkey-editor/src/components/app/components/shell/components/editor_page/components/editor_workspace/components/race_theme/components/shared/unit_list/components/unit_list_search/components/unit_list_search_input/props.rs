use super::view::UnitListSearchInputView;
use dioxus::prelude::*;

/// The search input's bound value plus its placeholder and the two handlers the
/// container wires up (debounced input, and Escape/Enter keydown).
#[derive(Props, Clone, PartialEq)]
pub struct UnitListSearchInputProps {
    pub value: ReadSignal<String>,
    pub placeholder: &'static str,
    pub on_input: EventHandler<FormEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl From<&UnitListSearchInputView> for UnitListSearchInputProps {
    fn from(view: &UnitListSearchInputView) -> Self {
        let UnitListSearchInputView {
            value,
            placeholder,
            on_input,
            on_keydown,
        } = view.clone();
        Self {
            value,
            placeholder,
            on_input,
            on_keydown,
        }
    }
}

impl ddd::Props for UnitListSearchInputProps {
    type View = UnitListSearchInputView;
}
