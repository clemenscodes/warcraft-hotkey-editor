use dioxus::prelude::*;

/// The track wraps the category sections passed as children.
#[derive(Props, Clone, PartialEq)]
pub struct UnitListTrackProps {
    pub children: Element,
}
