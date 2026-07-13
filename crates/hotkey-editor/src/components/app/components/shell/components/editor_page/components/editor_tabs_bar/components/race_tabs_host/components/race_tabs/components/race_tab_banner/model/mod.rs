use super::view::RaceTabBannerView;
use dioxus::prelude::*;
use warcraft_api::Race;

/// One race tab's input: which race it is (so the dispatcher picks that race's themed
/// wrapper) plus the finished binding — active state, display label, and the baked
/// pointer/keyboard handlers. The race comes from the domain `AllRaces` set the parent
/// iterates; no navigation signal is threaded, only the finished handlers.
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
