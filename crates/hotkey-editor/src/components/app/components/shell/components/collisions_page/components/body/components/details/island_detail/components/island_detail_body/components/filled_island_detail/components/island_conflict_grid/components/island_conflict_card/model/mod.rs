use super::view::IslandConflictCardView;
use crate::components::app::components::shell::components::collisions_page::presentation::ConflictView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictCardModel {
    pub conflict: ConflictView,
}

impl From<&IslandConflictCardView> for IslandConflictCardModel {
    fn from(view: &IslandConflictCardView) -> Self {
        let IslandConflictCardView { conflict } = view.clone();
        Self { conflict }
    }
}

impl ddd::Layered for IslandConflictCardModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for IslandConflictCardModel {
    type View = IslandConflictCardView;
}
