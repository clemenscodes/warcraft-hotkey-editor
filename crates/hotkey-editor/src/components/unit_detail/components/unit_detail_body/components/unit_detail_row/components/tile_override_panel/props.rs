use dioxus::prelude::*;

/// The override panel wraps its heading and the override card (or placeholder)
/// passed as children.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverridePanelProps {
    pub children: Element,
}
