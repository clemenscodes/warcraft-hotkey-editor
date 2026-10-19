use super::view::ActiveAccentView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ActiveAccentModel {
    pub hidden: bool,
}

impl From<&ActiveAccentView> for ActiveAccentModel {
    fn from(view: &ActiveAccentView) -> Self {
        let ActiveAccentView { hidden } = *view;
        Self { hidden }
    }
}

impl ddd::Layered for ActiveAccentModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for ActiveAccentModel {
    type View = ActiveAccentView;
}
