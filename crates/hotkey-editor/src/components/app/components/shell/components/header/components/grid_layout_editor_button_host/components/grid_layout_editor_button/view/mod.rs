use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct GridLayoutEditorButtonView {
    pub is_open: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for GridLayoutEditorButtonView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for GridLayoutEditorButtonView {}
