use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SegmentChoice {
    pub key: &'static str,
    pub label: &'static str,
    pub is_active: bool,
    pub on_pick: EventHandler<MouseEvent>,
}

#[derive(Clone, PartialEq)]
pub struct SegmentedControlView {
    pub options: Vec<SegmentChoice>,
}

impl ddd::View for SegmentedControlView {}
