use dioxus::prelude::*;
use warcraft_keybinds::CollisionSummary;

/// The collision button paints from a domain-counted summary and a click handler,
/// both supplied by `CollisionsButtonHost`; the leaf fetches nothing itself.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionsButtonProps {
    pub summary: CollisionSummary,
    pub onclick: EventHandler<MouseEvent>,
}
