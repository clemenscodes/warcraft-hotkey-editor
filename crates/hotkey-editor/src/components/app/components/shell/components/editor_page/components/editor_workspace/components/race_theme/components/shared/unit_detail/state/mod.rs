use super::presentation::{UnitCommandGridSlots, UnitOverrideTarget};
use warcraft_api::{HeroAttributes, UnitCombat, WarcraftObjectId};
use warcraft_keybinds::Evasion;

/// The panel's shaped view: either an empty-state message, or the fully-resolved domain
/// data for the loaded unit.
pub(super) enum UnitDetailView {
    Empty(&'static str),
    Loaded(Box<UnitDetailModel>),
}

/// A loaded unit's resolved domain data, distributed to the panel's children as named
/// fields. Holds domain values only — never a child's props.
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
    pub(super) override_target: UnitOverrideTarget,
}
