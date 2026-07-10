use dioxus::prelude::*;

/// The published `View` contract mirroring [`UnitListSearchProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitListSearchView {
    pub value: ReadSignal<String>,
    pub placeholder: &'static str,
    pub on_input: EventHandler<FormEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl ddd::View for UnitListSearchView {}
