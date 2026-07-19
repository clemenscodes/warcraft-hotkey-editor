use super::view::RaceScopeBackButtonView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct RaceScopeBackButtonModel {
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&RaceScopeBackButtonView> for RaceScopeBackButtonModel {
    fn from(view: &RaceScopeBackButtonView) -> Self {
        let RaceScopeBackButtonView { onclick } = view.clone();
        Self { onclick }
    }
}

impl ddd::Model for RaceScopeBackButtonModel {
    type View = RaceScopeBackButtonView;
}
