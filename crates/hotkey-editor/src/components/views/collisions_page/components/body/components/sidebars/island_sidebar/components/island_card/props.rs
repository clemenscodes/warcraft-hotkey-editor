use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// A selectable island collision card: its selected state, key, click handler, and
/// the data that fills it (the highlighted coordinate and the collision-count line).
#[derive(Props, Clone, PartialEq)]
pub struct IslandCardProps {
    pub is_selected: bool,
    #[props(into)]
    pub collision_key: String,
    pub onclick: EventHandler<MouseEvent>,
    pub coordinate: GridCoordinate,
    pub count: usize,
}
