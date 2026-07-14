use super::view::UnresolvedSectionView;
use crate::components::app::components::shell::components::resolve_page::presentation::UnresolvedView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UnresolvedSectionModel {
    pub unresolved: Vec<UnresolvedView>,
}

impl From<&UnresolvedSectionView> for UnresolvedSectionModel {
    fn from(view: &UnresolvedSectionView) -> Self {
        let UnresolvedSectionView { unresolved } = view.clone();
        Self { unresolved }
    }
}

impl ddd::Model for UnresolvedSectionModel {
    type View = UnresolvedSectionView;
}
