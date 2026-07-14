use super::view::AbilityTierButtonView;
use dioxus::prelude::*;

/// A tier-cycling arrow button: its accessible label, the inline arrow SVG, and the
/// click handler.
#[derive(Props, Clone, PartialEq)]
pub struct AbilityTierButtonModel {
    pub aria_label: &'static str,
    pub icon: &'static str,
    pub on_click: EventHandler<MouseEvent>,
}

impl From<&AbilityTierButtonView> for AbilityTierButtonModel {
    fn from(view: &AbilityTierButtonView) -> Self {
        let AbilityTierButtonView {
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

impl ddd::Model for AbilityTierButtonModel {
    type View = AbilityTierButtonView;
}
