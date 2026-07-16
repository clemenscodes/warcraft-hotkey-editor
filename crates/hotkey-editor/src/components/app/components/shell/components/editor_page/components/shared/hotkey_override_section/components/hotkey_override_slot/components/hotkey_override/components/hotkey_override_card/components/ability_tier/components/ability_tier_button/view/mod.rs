use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct AbilityTierButtonView {
    pub aria_label: &'static str,
    pub icon: &'static str,
    pub on_click: EventHandler<MouseEvent>,
}

impl ddd::View for AbilityTierButtonView {}
