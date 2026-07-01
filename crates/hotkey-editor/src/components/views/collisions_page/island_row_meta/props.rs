use dioxus::prelude::*;

/// The text column of a collision card (coords/name + collision count).
#[derive(Props, Clone, PartialEq)]
pub struct IslandRowMetaProps {
    pub children: Element,
}
