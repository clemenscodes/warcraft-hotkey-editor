use super::view::RaceScopeBackdropView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct RaceScopeBackdropModel {
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&RaceScopeBackdropView> for RaceScopeBackdropModel {
    fn from(view: &RaceScopeBackdropView) -> Self {
        let RaceScopeBackdropView { onclick } = view.clone();
        Self { onclick }
    }
}

impl ddd::Model for RaceScopeBackdropModel {
    type View = RaceScopeBackdropView;
}
