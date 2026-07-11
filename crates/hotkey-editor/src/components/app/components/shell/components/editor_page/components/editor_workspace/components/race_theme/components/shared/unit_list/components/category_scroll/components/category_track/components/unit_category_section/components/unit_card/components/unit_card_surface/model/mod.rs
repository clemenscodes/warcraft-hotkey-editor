use super::view::UnitCardSurfaceView;
use crate::components::app::components::shell::components::shared::icons::IconUrl;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The unit card's selectable button surface's input: the portrait plus name/id it lays
/// out, the selected flag the dispatcher reads to pick the look, and the click/keydown
/// handlers the card wires onto its button. The accent colour is read from
/// `--race-color`, so no race is threaded in.
#[derive(Props, Clone, PartialEq)]
pub struct UnitCardSurfaceModel {
    pub icon_path: Option<IconUrl>,
    #[props(into)]
    pub display_name: String,
    pub unit_id: WarcraftObjectId,
    pub is_selected: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&UnitCardSurfaceView> for UnitCardSurfaceModel {
    fn from(view: &UnitCardSurfaceView) -> Self {
        let UnitCardSurfaceView {
            icon_path,
            display_name,
            unit_id,
            is_selected,
            onclick,
            onkeydown,
        } = view.clone();
        Self {
            icon_path,
            display_name,
            unit_id,
            is_selected,
            onclick,
            onkeydown,
        }
    }
}

impl ddd::Model for UnitCardSurfaceModel {
    type View = UnitCardSurfaceView;
}
