use dioxus::prelude::*;

/// A tier-cycling arrow button: its accessible label, the inline arrow SVG, and the
/// click handler.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideTierButtonProps {
    pub aria_label: &'static str,
    pub icon: &'static str,
    pub on_click: EventHandler<MouseEvent>,
}
