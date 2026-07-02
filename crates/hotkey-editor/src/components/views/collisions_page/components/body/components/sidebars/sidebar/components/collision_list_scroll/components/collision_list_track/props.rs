use dioxus::prelude::*;

/// The track wraps the collision cards passed as children.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionListTrackProps {
    pub children: Element,
}
