use super::view::ResolvePagerView;
use crate::components::app::components::shell::components::resolve_page::presentation::{
    MoveSection, UnresolvedView,
};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ResolvePagerModel {
    pub section: Option<MoveSection>,
    pub unresolved: Vec<UnresolvedView>,
}

impl From<&ResolvePagerView> for ResolvePagerModel {
    fn from(view: &ResolvePagerView) -> Self {
        let ResolvePagerView {
            section,
            unresolved,
        } = view.clone();
        Self {
            section,
            unresolved,
        }
    }
}

impl ddd::Model for ResolvePagerModel {
    type View = ResolvePagerView;
}
