use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ResolveApplyBarView {
    pub moves_text: String,
    pub unresolved_count: usize,
    pub running: bool,
    pub on_apply: EventHandler<MouseEvent>,
}

impl ddd::Layered for ResolveApplyBarView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ResolveApplyBarView {}
