use dioxus::prelude::*;

/// The search box's bound value, placeholder, and the input/keydown handlers.
#[derive(Props, Clone, PartialEq)]
pub struct UnitListSearchProps {
    pub value: ReadSignal<String>,
    pub placeholder: &'static str,
    pub on_input: EventHandler<FormEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
}
