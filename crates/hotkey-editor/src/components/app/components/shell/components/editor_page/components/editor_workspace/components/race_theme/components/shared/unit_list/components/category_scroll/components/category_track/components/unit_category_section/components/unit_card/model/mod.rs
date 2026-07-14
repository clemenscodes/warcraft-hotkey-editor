use super::view::UnitCardView;
use crate::components::app::components::shell::components::shared::icons::IconUrl;
use dioxus::prelude::*;
use warcraft_api::{UnitKind, WarcraftObjectId};

#[derive(Props, Clone, PartialEq)]
pub struct UnitCardModel {
    pub unit_id: WarcraftObjectId,
    pub display_name: String,
    pub icon_path: Option<IconUrl>,
    pub unit_kind: UnitKind,
}

impl From<&UnitCardView> for UnitCardModel {
    fn from(view: &UnitCardView) -> Self {
        let UnitCardView {
            unit_id,
            display_name,
            icon_path,
            unit_kind,
        } = view.clone();
        Self {
            unit_id,
            display_name,
            icon_path,
            unit_kind,
        }
    }
}

impl ddd::Model for UnitCardModel {
    type View = UnitCardView;
}
