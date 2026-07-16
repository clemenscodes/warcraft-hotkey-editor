use super::presentation::UnitCommandGridSlots;
use crate::services::customkeys::queries::unit_override_target_query::UnitOverrideTargetView;
use warcraft_api::Evasion;
use warcraft_api::{HeroAttributes, UnitCombat, WarcraftObjectId};

pub(super) enum UnitDetailView {
    Empty(&'static str),
    Loaded(Box<UnitDetailModel>),
}

pub(super) struct UnitDetailModel {
    pub(super) unit_name: &'static str,
    pub(super) unit_id: WarcraftObjectId,
    pub(super) portrait_url: Option<String>,
    pub(super) has_hero_attributes: bool,
    pub(super) description_text: String,
    pub(super) combat: UnitCombat,
    pub(super) hero_attributes: Option<HeroAttributes>,
    pub(super) evasion: Evasion,
    pub(super) grid_slots: UnitCommandGridSlots,
    pub(super) override_target: UnitOverrideTargetView,
}
