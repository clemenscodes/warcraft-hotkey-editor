use crate::components::app::components::shell::components::shared::icons::IconUrl;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The unit card's selectable button surface's input: the portrait plus name/id it lays
/// out, the selected flag the dispatcher reads to pick the look, and the
/// click/keydown/mount handlers the card wires onto its button. The accent colour is
/// read from `--race-accent`, so no race is threaded in.
#[derive(Props, Clone, PartialEq)]
pub struct UnitCardSurfaceProps {
    pub icon_path: Option<IconUrl>,
    #[props(into)]
    pub display_name: String,
    pub unit_id: WarcraftObjectId,
    pub is_selected: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
    pub onmounted: EventHandler<Event<MountedData>>,
}
