use dioxus::prelude::*;

/// The scroll region wraps the track (its children) of category sections.
#[derive(Props, Clone, PartialEq)]
pub struct UnitListScrollProps {
    pub children: Element,
}
