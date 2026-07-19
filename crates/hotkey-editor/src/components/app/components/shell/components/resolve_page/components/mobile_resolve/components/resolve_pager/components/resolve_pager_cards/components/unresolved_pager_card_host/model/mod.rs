use super::view::UnresolvedPagerCardHostView;
use crate::components::app::components::shell::components::resolve_page::presentation::UnresolvedView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UnresolvedPagerCardHostModel {
    pub unresolved_view: UnresolvedView,
}

impl From<&UnresolvedPagerCardHostView> for UnresolvedPagerCardHostModel {
    fn from(view: &UnresolvedPagerCardHostView) -> Self {
        let UnresolvedPagerCardHostView { unresolved_view } = view.clone();
        Self { unresolved_view }
    }
}

impl ddd::Model for UnresolvedPagerCardHostModel {
    type View = UnresolvedPagerCardHostView;
}
