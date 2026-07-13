use dioxus::prelude::*;
use warcraft_api::Race;

/// The published `View` contract mirroring [`RaceTabBannerModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct RaceTabBannerView {
    pub race: Race,
    pub is_active: bool,
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl ddd::View for RaceTabBannerView {}
