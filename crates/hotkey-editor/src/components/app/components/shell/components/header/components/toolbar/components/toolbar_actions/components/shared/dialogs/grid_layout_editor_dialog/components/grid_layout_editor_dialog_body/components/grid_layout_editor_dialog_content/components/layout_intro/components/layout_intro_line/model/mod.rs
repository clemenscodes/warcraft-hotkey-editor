use super::view::LayoutIntroLineView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LayoutIntroLineModel {
    pub line: String,
}

impl From<&LayoutIntroLineView> for LayoutIntroLineModel {
    fn from(view: &LayoutIntroLineView) -> Self {
        let LayoutIntroLineView { line } = view.clone();
        Self { line }
    }
}

impl ddd::Layered for LayoutIntroLineModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for LayoutIntroLineModel {
    type View = LayoutIntroLineView;
}
