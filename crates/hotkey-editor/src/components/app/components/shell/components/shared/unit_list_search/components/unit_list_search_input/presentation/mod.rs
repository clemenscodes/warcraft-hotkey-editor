use super::model::UnitListSearchInputModel;
use dioxus::prelude::*;

pub(super) struct UnitListSearchInputPresentation {
    pub(super) value: ReadSignal<String>,
    pub(super) placeholder: &'static str,
    pub(super) on_input: EventHandler<FormEvent>,
    pub(super) on_keydown: EventHandler<KeyboardEvent>,
}

impl ddd::Layered for UnitListSearchInputPresentation {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Presentation for UnitListSearchInputPresentation {
    type Model = UnitListSearchInputModel;
}

pub(super) fn use_unit_list_search_input(
    props: &UnitListSearchInputModel,
) -> UnitListSearchInputPresentation {
    let value = props.value;
    let placeholder = props.placeholder;
    let on_input = props.on_input;
    let on_keydown = props.on_keydown;
    UnitListSearchInputPresentation {
        value,
        placeholder,
        on_input,
        on_keydown,
    }
}
