use dioxus::prelude::*;

/// A selectable unit collision card: its selected state, key, click handler, and
/// the data that fills it (portrait, name, object id, and collision-count line).
#[derive(Props, Clone, PartialEq)]
pub struct UnitCardProps {
    pub is_selected: bool,
    #[props(into)]
    pub collision_key: String,
    pub onclick: EventHandler<MouseEvent>,
    pub icon_url: Option<String>,
    #[props(into)]
    pub name: String,
    #[props(into)]
    pub unit_id: String,
    pub count: usize,
}
