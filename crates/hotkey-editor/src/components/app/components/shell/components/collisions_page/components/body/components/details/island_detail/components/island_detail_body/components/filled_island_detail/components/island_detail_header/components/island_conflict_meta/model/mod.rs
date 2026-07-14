use super::view::IslandConflictMetaView;
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictMetaModel {
    pub coordinate: GridCoordinate,
    pub count: usize,
}

impl From<&IslandConflictMetaView> for IslandConflictMetaModel {
    fn from(view: &IslandConflictMetaView) -> Self {
        let IslandConflictMetaView { coordinate, count } = view.clone();
        Self { coordinate, count }
    }
}

impl ddd::Model for IslandConflictMetaModel {
    type View = IslandConflictMetaView;
}
