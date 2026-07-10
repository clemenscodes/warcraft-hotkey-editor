use crate::components::app::components::shell::components::collisions_page::logic::IslandView;
use dioxus::prelude::*;

/// The populated island detail pane: the selected island, whose coordinate heads the
/// pane over its per-unit conflict cards.
#[derive(Props, Clone, PartialEq)]
pub struct FilledIslandDetailProps {
    pub island: IslandView,
}
