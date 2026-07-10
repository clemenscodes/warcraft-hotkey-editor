use crate::components::app::components::shell::components::shared::icons::IconUrl;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The idle unit card surface's props: the portrait and text it lays out plus the
/// card's handlers. Built by the dispatcher from `UnitCardSurfaceProps`.
#[derive(Props, Clone, PartialEq)]
pub struct IdleUnitCardSurfaceProps {
    pub icon_path: Option<IconUrl>,
    #[props(into)]
    pub display_name: String,
    pub unit_id: WarcraftObjectId,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}
