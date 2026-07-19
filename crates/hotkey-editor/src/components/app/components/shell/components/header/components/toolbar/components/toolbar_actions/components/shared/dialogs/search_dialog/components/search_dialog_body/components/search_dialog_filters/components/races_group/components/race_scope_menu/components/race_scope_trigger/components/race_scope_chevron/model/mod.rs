use super::view::RaceScopeChevronView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct RaceScopeChevronModel {
    pub is_open: bool,
}

impl From<&RaceScopeChevronView> for RaceScopeChevronModel {
    fn from(view: &RaceScopeChevronView) -> Self {
        let RaceScopeChevronView { is_open } = view.clone();
        Self { is_open }
    }
}

impl ddd::Model for RaceScopeChevronModel {
    type View = RaceScopeChevronView;
}
