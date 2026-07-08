use super::super::super::state::CollisionCardContent;
use super::components::collision_card_meta::CollisionCardMetaProps;
use super::components::collision_card_visual::CollisionCardVisualProps;
use dioxus::prelude::*;

/// The collision card's selectable button surface: the leading visual (unit portrait
/// or island mini grid) beside the meta line and count, the selected flag that drives
/// its fixed collision-gold accent, and the click handler the card wires onto its own
/// button.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionCardSurfaceProps {
    pub is_selected: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub count: usize,
    pub content: CollisionCardContent,
}

impl From<&CollisionCardSurfaceProps> for CollisionCardVisualProps {
    fn from(props: &CollisionCardSurfaceProps) -> Self {
        let content = props.content.clone();
        Self { content }
    }
}

impl From<&CollisionCardSurfaceProps> for CollisionCardMetaProps {
    fn from(props: &CollisionCardSurfaceProps) -> Self {
        let content = props.content.clone();
        let count = props.count;
        Self { content, count }
    }
}
