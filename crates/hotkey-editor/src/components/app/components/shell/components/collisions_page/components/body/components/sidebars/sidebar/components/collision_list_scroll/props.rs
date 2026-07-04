use dioxus::prelude::*;

/// The collision list scroll wraps the fed-in cards.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionListScrollProps {
    pub children: Element,
}
