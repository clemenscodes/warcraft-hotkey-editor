use dioxus::prelude::*;

/// A selectable collision-list card, sized like the editor's unit cards. Holds the
/// card's content (icon/grid + meta), its selected state, its key, and the click
/// handler that selects it.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionCardProps {
    pub is_selected: bool,
    #[props(into)]
    pub collision_key: String,
    pub onclick: EventHandler<MouseEvent>,
    pub children: Element,
}
