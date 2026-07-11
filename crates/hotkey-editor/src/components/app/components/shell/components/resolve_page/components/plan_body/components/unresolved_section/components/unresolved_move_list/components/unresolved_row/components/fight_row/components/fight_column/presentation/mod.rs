use crate::components::app::components::shell::components::resolve_page::presentation::UnresolvedView;
use crate::services::carriers::InspectedAbility;
use warcraft_api::WarcraftObjectId;

/// The stuck column's shaped state: the name-plate fields and the ability-icon fields. A
/// stuck ability never wins a cell, so its icon is not ringed.
pub(super) struct FightColumnPresentation {
    pub(super) name: String,
    pub(super) object_id: WarcraftObjectId,
    pub(super) icon_url: Option<String>,
    pub(super) carrier_count: usize,
    pub(super) disabled: bool,
    pub(super) inspected: InspectedAbility,
}

impl From<&UnresolvedView> for FightColumnPresentation {
    fn from(unresolved_view: &UnresolvedView) -> Self {
        let ability = unresolved_view.ability();
        let name = ability.name().to_owned();
        let object_id = ability.object_id();
        let icon_url = ability.icon_url().map(str::to_owned);
        let carrier_count = unresolved_view.carrier_count();
        let carrier_unit_ids_ref = unresolved_view.carrier_unit_ids();
        let carrier_unit_ids = carrier_unit_ids_ref.to_vec();
        let disabled = carrier_unit_ids.is_empty();
        let name_for_inspected = name.clone();
        let inspected = InspectedAbility::new(name_for_inspected, carrier_unit_ids);
        Self {
            name,
            object_id,
            icon_url,
            carrier_count,
            disabled,
            inspected,
        }
    }
}
