use super::view::MoveListView;
use crate::components::app::components::shell::components::resolve_page::presentation::MoveSection;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MoveListModel {
    pub section: Option<MoveSection>,
}

impl From<&MoveListView> for MoveListModel {
    fn from(view: &MoveListView) -> Self {
        let MoveListView { section } = view.clone();
        Self { section }
    }
}

impl ddd::Model for MoveListModel {
    type View = MoveListView;
}
