use super::props::UnitPositionConflictCardProps;
use crate::components::views::collisions_page::conflict_ability::ConflictAbilityProps;

/// The shaped card: the caption, the colliding cell, and the abilities that land on
/// it. A two-way clash flanks the cell; a rarer 3+ way clash stacks the cell above.
pub(super) struct UnitPositionConflictCardModel {
    pub(super) role_label: String,
    pub(super) position_column: u8,
    pub(super) position_row: u8,
    pub(super) is_pair: bool,
    pub(super) abilities: Vec<ConflictAbilityProps>,
}

impl From<&UnitPositionConflictCardProps> for UnitPositionConflictCardModel {
    fn from(props: &UnitPositionConflictCardProps) -> Self {
        let role_label = props.conflict.role_label().to_owned();
        let position_column = props.conflict.position_column();
        let position_row = props.conflict.position_row();
        let abilities: Vec<ConflictAbilityProps> = props
            .conflict
            .abilities()
            .iter()
            .map(|ability| ConflictAbilityProps {
                ability_name: ability.name().to_owned(),
                ability_id: ability.object_id().to_owned(),
                icon_url: ability.icon_url().map(str::to_owned),
                unit_id: props.unit_id.clone(),
                view_navigation: props.view_navigation,
            })
            .collect();
        let is_pair = abilities.len() == 2;
        Self {
            role_label,
            position_column,
            position_row,
            is_pair,
            abilities,
        }
    }
}
