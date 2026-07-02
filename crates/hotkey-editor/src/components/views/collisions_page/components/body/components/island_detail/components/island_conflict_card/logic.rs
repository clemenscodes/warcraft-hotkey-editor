use super::super::island_conflict_ability::IslandConflictAbilityProps;
use super::super::island_conflict_unit::IslandConflictUnitProps;
use super::props::IslandConflictCardProps;

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
        let own_ability_view = own.ability();
        let own_ability = IslandConflictAbilityProps {
            ability_name: own_ability_view.name().to_owned(),
            ability_id: own_ability_view.object_id().to_owned(),
            icon_url: own_ability_view.icon_url().map(str::to_owned),
            extra_count: own.extra_count(),
            carrier_unit_ids: own.carrier_unit_ids().to_vec(),
            carrier_dialog: props.carrier_dialog,
        };
        let shared = conflict.shared_ability();
        let shared_ability_view = shared.ability();
        let shared_ability = IslandConflictAbilityProps {
            ability_name: shared_ability_view.name().to_owned(),
            ability_id: shared_ability_view.object_id().to_owned(),
            icon_url: shared_ability_view.icon_url().map(str::to_owned),
            extra_count: shared.extra_count(),
            carrier_unit_ids: shared.carrier_unit_ids().to_vec(),
            carrier_dialog: props.carrier_dialog,
        };
        Self {
            unit,
            own_ability,
            shared_ability,
        }
    }
}
