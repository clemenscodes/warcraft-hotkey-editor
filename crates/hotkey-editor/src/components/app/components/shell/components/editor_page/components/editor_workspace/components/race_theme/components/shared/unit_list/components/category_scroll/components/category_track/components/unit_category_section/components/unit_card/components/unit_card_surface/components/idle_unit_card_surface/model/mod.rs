use super::view::IdleUnitCardSurfaceView;
use crate::components::app::components::shell::components::shared::icons::IconUrl;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The idle unit card surface's props: the portrait and text it lays out plus the
/// card's handlers. Built by the dispatcher from `UnitCardSurfaceModel`.
#[derive(Props, Clone, PartialEq)]
pub struct IdleUnitCardSurfaceModel {
    pub icon_path: Option<IconUrl>,
    #[props(into)]
    pub display_name: String,
    pub unit_id: WarcraftObjectId,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&IdleUnitCardSurfaceView> for IdleUnitCardSurfaceModel {
    fn from(view: &IdleUnitCardSurfaceView) -> Self {
        let IdleUnitCardSurfaceView {
            icon_path,
            display_name,
            unit_id,
            onclick,
            onkeydown,
        } = view.clone();
        Self {
            icon_path,
            display_name,
            unit_id,
            onclick,
            onkeydown,
        }
    }
}

impl ddd::Model for IdleUnitCardSurfaceModel {
    type View = IdleUnitCardSurfaceView;
}
