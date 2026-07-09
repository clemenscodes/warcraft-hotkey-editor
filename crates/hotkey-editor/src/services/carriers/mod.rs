use crate::components::app::components::shell::components::shared::icons::ResolvedIcon;
use warcraft_api::WarcraftObjectId;

/// The identity of the ability whose carriers are to be shown: its display name and
/// the ids of the units that carry it (the query input). A lean value a trigger stashes
/// into its own open-state signal — not the resolved carrier data.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InspectedAbility {
    ability_name: String,
    carrier_unit_ids: Vec<WarcraftObjectId>,
}

impl InspectedAbility {
    pub(crate) fn new(ability_name: String, carrier_unit_ids: Vec<WarcraftObjectId>) -> Self {
        Self {
            ability_name,
            carrier_unit_ids,
        }
    }

    pub(crate) fn ability_name(&self) -> &str {
        &self.ability_name
    }

    pub(crate) fn carrier_unit_ids(&self) -> &[WarcraftObjectId] {
        &self.carrier_unit_ids
    }
}

/// One unit resolved to its id, display name, and icon. The id is kept so a click can
/// deep-link into the editor focused on that unit.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CarrierUnitView {
    unit_id: WarcraftObjectId,
    name: String,
    icon_url: Option<String>,
}

impl From<WarcraftObjectId> for CarrierUnitView {
    fn from(unit_id: WarcraftObjectId) -> Self {
        let unit_id_value = unit_id.value();
        let resolved = ResolvedIcon::lookup(unit_id_value);
        let icon_url = resolved.icon_url().map(str::to_owned);
        let name = resolved.name_or(unit_id_value);
        Self {
            unit_id,
            name,
            icon_url,
        }
    }
}

impl CarrierUnitView {
    pub(crate) fn unit_id(&self) -> WarcraftObjectId {
        self.unit_id
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn icon_url(&self) -> Option<&str> {
        self.icon_url.as_deref()
    }
}

/// The page-agnostic carriers query: given the ids of the units that carry an ability,
/// resolve each to a display view. It knows nothing about where it is called from or
/// how the result is shown — only the data. Both the collisions and resolve pages feed
/// the carriers dialog from it.
pub struct Carriers;

impl Carriers {
    pub(crate) fn for_unit_ids(carrier_unit_ids: &[WarcraftObjectId]) -> Vec<CarrierUnitView> {
        let mut views: Vec<CarrierUnitView> = Vec::with_capacity(carrier_unit_ids.len());
        for carrier_unit_id in carrier_unit_ids {
            let carrier_unit_id = *carrier_unit_id;
            let view = CarrierUnitView::from(carrier_unit_id);
            views.push(view);
        }
        views
    }
}
