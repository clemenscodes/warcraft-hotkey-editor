use super::view::ActiveStatGainView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ActiveStatGainModel {
    #[props(into)]
    pub text: String,
}

impl From<&ActiveStatGainView> for ActiveStatGainModel {
    fn from(view: &ActiveStatGainView) -> Self {
        let ActiveStatGainView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Layered for ActiveStatGainModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for ActiveStatGainModel {
    type View = ActiveStatGainView;
}
