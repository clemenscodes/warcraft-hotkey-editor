use super::components::island_conflict_ability::IslandConflictAbilityProps;
use super::components::island_conflict_unit::IslandConflictUnitProps;
use super::props::IslandConflictCardProps;
use crate::components::app::components::shell::components::collisions_page::logic::ConflictAbilityView;
use crate::services::carriers::InspectedAbility;

/// The shaped card: the affected unit heading it, and the two abilities that clash.
pub(super) struct IslandConflictCardModel {
    pub(super) unit: IslandConflictUnitProps,
    pub(super) own_ability: IslandConflictAbilityProps,
    pub(super) shared_ability: IslandConflictAbilityProps,
}

impl From<&IslandConflictCardProps> for IslandConflictCardModel {
    fn from(props: &IslandConflictCardProps) -> Self {
        let conflict = &props.conflict;
        let affected_unit = conflict.unit();
        let unit = IslandConflictUnitProps {
            unit_id: affected_unit.unit_id().to_owned(),
            icon_url: affected_unit.icon_url().map(str::to_owned),
            name: affected_unit.name().to_owned(),
            view_navigation: props.view_navigation,
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

impl IslandConflictCardModel {
    fn ability(view: &ConflictAbilityView) -> IslandConflictAbilityProps {
        let ability_view = view.ability();
        let ability_name = ability_view.name().to_owned();
        let ability_id = ability_view.object_id().to_owned();
        let icon_url = ability_view.icon_url().map(str::to_owned);
        let extra_count = view.extra_count();
        let carrier_unit_ids = view.carrier_unit_ids().to_vec();
        let inspected = InspectedAbility::new(ability_name.clone(), carrier_unit_ids);
        IslandConflictAbilityProps {
            ability_name,
            ability_id,
            icon_url,
            extra_count,
            inspected,
        }
    }
}
