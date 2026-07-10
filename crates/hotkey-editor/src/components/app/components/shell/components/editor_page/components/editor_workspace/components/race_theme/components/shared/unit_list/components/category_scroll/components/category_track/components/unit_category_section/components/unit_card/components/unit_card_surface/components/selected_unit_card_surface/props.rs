use super::view::SelectedUnitCardSurfaceView;
use crate::components::app::components::shell::components::shared::icons::IconUrl;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The selected unit card surface's props: the portrait and text it lays out plus the
/// card's handlers. Built by the dispatcher from `UnitCardSurfaceProps`.
#[derive(Props, Clone, PartialEq)]
pub struct SelectedUnitCardSurfaceProps {
    pub icon_path: Option<IconUrl>,
    #[props(into)]
    pub display_name: String,
    pub unit_id: WarcraftObjectId,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&SelectedUnitCardSurfaceView> for SelectedUnitCardSurfaceProps {
    fn from(view: &SelectedUnitCardSurfaceView) -> Self {
        let SelectedUnitCardSurfaceView {
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

impl ddd::Props for SelectedUnitCardSurfaceProps {
    type View = SelectedUnitCardSurfaceView;
}
