use super::model::IslandConflictCardModel;
use crate::components::app::components::shell::components::collisions_page::presentation::ConflictAbilityView;
use crate::services::carriers::InspectedAbility;
use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub(crate) struct IslandUnitData {
    pub(super) unit_id: WarcraftObjectId,
    pub(super) icon_url: Option<String>,
    pub(super) name: String,
}

#[derive(Clone, PartialEq)]
pub(crate) struct IslandAbilityData {
    pub(super) ability_name: String,
    pub(super) ability_id: WarcraftObjectId,
    pub(super) icon_url: Option<String>,
    pub(super) extra_count: usize,
    pub(super) inspected: InspectedAbility,
}

pub(super) struct IslandConflictCardPresentation {
    pub(super) unit: IslandUnitData,
    pub(super) own_ability: IslandAbilityData,
    pub(super) shared_ability: IslandAbilityData,
}

impl From<&IslandConflictCardModel> for IslandConflictCardPresentation {
    fn from(props: &IslandConflictCardModel) -> Self {
        let conflict = &props.conflict;
        let affected_unit = conflict.unit();
        let unit_id = affected_unit.unit_id();
        let icon_url = affected_unit.icon_url().map(str::to_owned);
        let name = affected_unit.name().to_owned();
        let unit = IslandUnitData {
            unit_id,
            icon_url,
            name,
        };
        let own = conflict.own_ability();
        let own_ability = Self::ability(own);
        let shared = conflict.shared_ability();
        let shared_ability = Self::ability(shared);
        Self {
            unit,
            own_ability,
            shared_ability,
        }
    }
}

impl IslandConflictCardPresentation {
    fn ability(view: &ConflictAbilityView) -> IslandAbilityData {
        let ability_view = view.ability();
        let ability_name = ability_view.name().to_owned();
        let ability_id = ability_view.object_id();
        let icon_url = ability_view.icon_url().map(str::to_owned);
        let extra_count = view.extra_count();
        let carrier_unit_ids = view.carrier_unit_ids().to_vec();
        let inspected = InspectedAbility::new(ability_name.clone(), carrier_unit_ids);
        IslandAbilityData {
            ability_name,
            ability_id,
            icon_url,
            extra_count,
            inspected,
        }
    }
}

impl ddd::Presentation for IslandConflictCardPresentation {
    type Model = IslandConflictCardModel;
}
