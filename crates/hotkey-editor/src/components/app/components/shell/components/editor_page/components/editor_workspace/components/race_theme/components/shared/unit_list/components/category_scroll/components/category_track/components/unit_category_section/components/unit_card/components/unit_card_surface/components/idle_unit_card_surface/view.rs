use crate::components::app::components::shell::components::shared::icons::IconUrl;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The published `View` contract mirroring [`IdleUnitCardSurfaceProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct IdleUnitCardSurfaceView {
    pub icon_path: Option<IconUrl>,
    pub display_name: String,
    pub unit_id: WarcraftObjectId,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl ddd::View for IdleUnitCardSurfaceView {}
