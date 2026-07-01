use dioxus::prelude::*;
/// The column/row coordinate pair on an island card.
#[derive(Props, Clone, PartialEq)]
pub struct IslandCoordGroupProps {
    pub children: Element,
}
