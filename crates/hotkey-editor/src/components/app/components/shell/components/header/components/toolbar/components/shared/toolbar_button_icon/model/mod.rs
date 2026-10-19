use super::view::ToolbarButtonIconView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToolbarButtonIconModel {
    pub icon: &'static str,
}

impl From<&ToolbarButtonIconView> for ToolbarButtonIconModel {
    fn from(view: &ToolbarButtonIconView) -> Self {
        let ToolbarButtonIconView { icon } = view.clone();
        Self { icon }
    }
}

impl ddd::Layered for ToolbarButtonIconModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for ToolbarButtonIconModel {
    type View = ToolbarButtonIconView;
}
