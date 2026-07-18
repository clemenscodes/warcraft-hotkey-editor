use super::view::RaceTabBannerView;
use dioxus::prelude::*;
use warcraft_api::Race;

#[derive(Props, Clone, PartialEq)]
pub struct RaceTabBannerModel {
    pub race: Race,
    pub is_active: bool,
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&RaceTabBannerView> for RaceTabBannerModel {
    fn from(view: &RaceTabBannerView) -> Self {
        let RaceTabBannerView {
            race,
            is_active,
            label,
            onclick,
            onkeydown,
        } = view.clone();
        Self {
            race,
            is_active,
            label,
            onclick,
            onkeydown,
        }
    }
}

impl ddd::Model for RaceTabBannerModel {
    type View = RaceTabBannerView;
}
