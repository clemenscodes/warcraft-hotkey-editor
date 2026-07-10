use super::components::unit_list_search_input::UnitListSearchInputProps;
use dioxus::prelude::*;

/// The search box's bound value, placeholder, and the input/keydown handlers.
#[derive(Props, Clone, PartialEq)]
pub struct UnitListSearchProps {
    pub value: ReadSignal<String>,
    pub placeholder: &'static str,
    pub on_input: EventHandler<FormEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl From<&UnitListSearchProps> for UnitListSearchInputProps {
    fn from(props: &UnitListSearchProps) -> Self {
        let value = props.value;
        let placeholder = props.placeholder;
        let on_input = props.on_input;
        let on_keydown = props.on_keydown;
        Self {
            value,
            placeholder,
            on_input,
            on_keydown,
        }
    }
}
