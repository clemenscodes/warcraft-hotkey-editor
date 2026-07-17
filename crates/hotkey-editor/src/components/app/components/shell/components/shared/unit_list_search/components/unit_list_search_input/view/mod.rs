use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct UnitListSearchInputView {
    pub value: ReadSignal<String>,
    pub placeholder: &'static str,
    pub on_input: EventHandler<FormEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl ddd::View for UnitListSearchInputView {}
