use super::view::TileOverrideTierButtonView;
use dioxus::prelude::*;

/// A tier-cycling arrow button: its accessible label, the inline arrow SVG, and the
/// click handler.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideTierButtonProps {
    pub aria_label: &'static str,
    pub icon: &'static str,
    pub on_click: EventHandler<MouseEvent>,
}

impl From<&TileOverrideTierButtonView> for TileOverrideTierButtonProps {
    fn from(view: &TileOverrideTierButtonView) -> Self {
        let TileOverrideTierButtonView {
            aria_label,
            icon,
            on_click,
        } = view.clone();
        Self {
            aria_label,
            icon,
            on_click,
        }
    }
}

impl ddd::Props for TileOverrideTierButtonProps {
    type View = TileOverrideTierButtonView;
}
