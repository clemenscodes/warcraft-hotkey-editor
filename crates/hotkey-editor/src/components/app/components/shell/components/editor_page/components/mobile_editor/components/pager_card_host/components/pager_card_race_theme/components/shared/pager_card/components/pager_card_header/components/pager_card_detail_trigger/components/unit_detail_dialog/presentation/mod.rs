use super::model::UnitDetailDialogModel;
use crate::components::app::components::shell::components::shared::icons::IconUrl;
use dioxus::prelude::*;
use warcraft_api::WarcraftApi;
use warcraft_api::{Evasion, HeroAttributes, UnitCombat, WarcraftObjectId, WarcraftObjectMeta};

#[derive(Clone, PartialEq)]
pub(super) struct ResolvedDialogUnit {
    pub(super) unit_name: &'static str,
    pub(super) portrait_url: Option<String>,
    pub(super) description_text: String,
    pub(super) combat: UnitCombat,
    pub(super) hero_attributes: Option<HeroAttributes>,
    pub(super) evasion: Evasion,
}

impl TryFrom<WarcraftObjectId> for ResolvedDialogUnit {
    type Error = &'static str;

    fn try_from(unit_id: WarcraftObjectId) -> Result<Self, Self::Error> {
        let api = WarcraftApi::default();
        let Some(unit_object) = api.object(unit_id) else {
            return Err("Unit not found in database.");
        };
        let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
            return Err("Selected object is not a unit.");
        };
        let unit_name = unit_object.names().first().copied().unwrap_or("(unnamed)");
        let portrait_url = unit_object
            .icons()
            .first()
            .copied()
            .map(IconUrl::from_database_path)
            .map(|url| url.to_string());
        let description_text = unit_object.ubertip().unwrap_or_default().to_string();
        let combat = *unit_meta.combat();
        let hero_attributes = unit_meta.hero_attributes().copied();
        let evasion = api.unit().evasion(unit_id);
        Ok(Self {
            unit_name,
            portrait_url,
            description_text,
            combat,
            hero_attributes,
            evasion,
        })
    }
}

pub(super) struct UnitDetailDialogPresentation {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) resolved: Option<ResolvedDialogUnit>,
}

impl ddd::Presentation for UnitDetailDialogPresentation {
    type Model = UnitDetailDialogModel;
}

pub(super) fn use_unit_detail_dialog(
    props: &UnitDetailDialogModel,
) -> UnitDetailDialogPresentation {
    let open = props.open;
    let on_open_change = props.on_open_change;
    let unit_id = props.unit_id;
    let resolved = if open {
        ResolvedDialogUnit::try_from(unit_id).ok()
    } else {
        None
    };
    UnitDetailDialogPresentation {
        open,
        on_open_change,
        resolved,
    }
}
