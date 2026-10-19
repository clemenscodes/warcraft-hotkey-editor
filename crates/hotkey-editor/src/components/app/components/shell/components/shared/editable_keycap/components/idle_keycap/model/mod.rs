use super::view::IdleKeycapView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct IdleKeycapModel {
    #[props(into)]
    pub label: String,
}

impl From<&IdleKeycapView> for IdleKeycapModel {
    fn from(view: &IdleKeycapView) -> Self {
        let IdleKeycapView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Layered for IdleKeycapModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for IdleKeycapModel {
    type View = IdleKeycapView;
}
