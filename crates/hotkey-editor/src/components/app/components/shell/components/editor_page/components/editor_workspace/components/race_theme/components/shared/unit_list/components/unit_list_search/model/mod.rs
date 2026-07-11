use super::view::UnitListSearchView;
use dioxus::prelude::*;

/// The search box's bound value, placeholder, and the input/keydown handlers.
#[derive(Props, Clone, PartialEq)]
pub struct UnitListSearchModel {
    pub value: ReadSignal<String>,
    pub placeholder: &'static str,
    pub on_input: EventHandler<FormEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl From<&UnitListSearchView> for UnitListSearchModel {
    fn from(view: &UnitListSearchView) -> Self {
        let UnitListSearchView {
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

impl ddd::Model for UnitListSearchModel {
    type View = UnitListSearchView;
}
