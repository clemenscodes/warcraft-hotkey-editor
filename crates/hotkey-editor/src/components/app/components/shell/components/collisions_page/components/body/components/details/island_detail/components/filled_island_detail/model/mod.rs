use super::view::FilledIslandDetailView;
use crate::components::app::components::shell::components::collisions_page::presentation::IslandView;
use dioxus::prelude::*;

/// The populated island detail pane: the selected island, whose coordinate heads the
/// pane over its per-unit conflict cards.
#[derive(Props, Clone, PartialEq)]
pub struct FilledIslandDetailModel {
    pub island: IslandView,
}

impl From<&FilledIslandDetailView> for FilledIslandDetailModel {
    fn from(view: &FilledIslandDetailView) -> Self {
        let FilledIslandDetailView { island } = view.clone();
        Self { island }
    }
}

impl ddd::Model for FilledIslandDetailModel {
    type View = FilledIslandDetailView;
}
