use super::view::UnresolvedSectionView;
use crate::components::app::components::shell::components::resolve_page::logic::UnresolvedView;
use dioxus::prelude::*;

/// The unresolved-abilities section: one stuck card per ability the cascade could
/// not place.
#[derive(Props, Clone, PartialEq)]
pub struct UnresolvedSectionProps {
    pub unresolved: Vec<UnresolvedView>,
}

impl From<&UnresolvedSectionView> for UnresolvedSectionProps {
    fn from(view: &UnresolvedSectionView) -> Self {
        let UnresolvedSectionView { unresolved } = view.clone();
        Self { unresolved }
    }
}

impl ddd::Props for UnresolvedSectionProps {
    type View = UnresolvedSectionView;
}
