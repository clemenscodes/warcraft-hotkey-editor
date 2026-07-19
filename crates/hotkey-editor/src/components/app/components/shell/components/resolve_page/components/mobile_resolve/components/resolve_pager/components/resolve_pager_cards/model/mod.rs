use super::view::ResolvePagerCardsView;
use crate::components::app::components::shell::components::resolve_page::presentation::{
    MoveSection, UnresolvedView,
};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ResolvePagerCardsModel {
    pub section: Option<MoveSection>,
    pub unresolved: Vec<UnresolvedView>,
}

impl From<&ResolvePagerCardsView> for ResolvePagerCardsModel {
    fn from(view: &ResolvePagerCardsView) -> Self {
        let ResolvePagerCardsView {
            section,
            unresolved,
        } = view.clone();
        Self {
            section,
            unresolved,
        }
    }
}

impl ddd::Model for ResolvePagerCardsModel {
    type View = ResolvePagerCardsView;
}
