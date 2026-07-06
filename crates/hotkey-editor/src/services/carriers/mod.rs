use crate::components::app::components::shell::components::shared::icons::IconUrl;
use warcraft_database::ObjectLookup;

/// The identity of the ability whose carriers are to be shown: its display name and
/// the ids of the units that carry it (the query input). A lean value a trigger stashes
/// into its own open-state signal — not the resolved carrier data.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InspectedAbility {
    ability_name: String,
    carrier_unit_ids: Vec<String>,
}

impl InspectedAbility {
    pub(crate) fn new(ability_name: String, carrier_unit_ids: Vec<String>) -> Self {
        Self {
            ability_name,
            carrier_unit_ids,
        }
    }

    pub(crate) fn ability_name(&self) -> &str {
        &self.ability_name
    }

    pub(crate) fn carrier_unit_ids(&self) -> &[String] {
        &self.carrier_unit_ids
    }
}

/// One unit resolved to its id, display name, and icon. The id is kept so a click can
/// deep-link into the editor focused on that unit.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CarrierUnitView {
    unit_id: String,
    name: String,
    icon_url: Option<String>,
}

impl CarrierUnitView {
    fn resolve(unit_id_value: &str) -> Self {
        let object_option = ObjectLookup::by_id(unit_id_value);
        let icon_url = object_option
            .and_then(|object| object.icons().first().copied())
            .map(IconUrl::from_database_path)
            .map(|icon| icon.to_string());
        let name_option = object_option.and_then(|object| object.names().first().copied());
        let name = name_option.unwrap_or(unit_id_value).to_owned();
        let unit_id = unit_id_value.to_owned();
        Self {
            unit_id,
            name,
            icon_url,
        }
    }

    pub(crate) fn unit_id(&self) -> &str {
        &self.unit_id
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
    pub(crate) fn for_unit_ids(carrier_unit_ids: &[String]) -> Vec<CarrierUnitView> {
        let mut views: Vec<CarrierUnitView> = Vec::with_capacity(carrier_unit_ids.len());
        for carrier_unit_id in carrier_unit_ids {
            let view = CarrierUnitView::resolve(carrier_unit_id);
            views.push(view);
        }
        views
    }
}
