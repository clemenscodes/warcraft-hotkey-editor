use super::view::UnitListSearchInputView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UnitListSearchInputModel {
    pub value: ReadSignal<String>,
    pub placeholder: &'static str,
    pub on_input: EventHandler<FormEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub autofocus: bool,
}

impl From<&UnitListSearchInputView> for UnitListSearchInputModel {
    fn from(view: &UnitListSearchInputView) -> Self {
        let UnitListSearchInputView {
            value,
            placeholder,
            on_input,
            on_keydown,
            autofocus,
        } = view.clone();
        Self {
            value,
            placeholder,
            on_input,
            on_keydown,
            autofocus,
        }
    }
}

impl ddd::Layered for UnitListSearchInputModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for UnitListSearchInputModel {
    type View = UnitListSearchInputView;
}
