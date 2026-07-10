use crate::components::app::components::shell::components::collisions_page::logic::UnitIconView;
use dioxus::prelude::*;

/// The detail-pane header row: the selected unit and its collision count. The header
/// builds the unit button and the text meta column from the unit view.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyDetailHeaderProps {
    pub unit: UnitIconView,
    pub count: usize,
}
