use super::view::FilledIslandDetailView;
use crate::components::app::components::shell::components::collisions_page::logic::IslandView;
use dioxus::prelude::*;

/// The populated island detail pane: the selected island, whose coordinate heads the
/// pane over its per-unit conflict cards.
#[derive(Props, Clone, PartialEq)]
pub struct FilledIslandDetailProps {
    pub island: IslandView,
}

impl From<&FilledIslandDetailView> for FilledIslandDetailProps {
    fn from(view: &FilledIslandDetailView) -> Self {
        let FilledIslandDetailView { island } = view.clone();
        Self { island }
    }
}

impl ddd::Props for FilledIslandDetailProps {
    type View = FilledIslandDetailView;
}
