use super::components::conflict_position_cell::ConflictPositionCellProps;
use super::components::position_multi_stack::PositionMultiStackProps;
use super::components::position_pair_row::{PositionPair, PositionPairRowProps};
use super::props::UnitPositionConflictCardProps;
use crate::components::views::collisions_page::components::body::components::conflict_ability::ConflictAbilityProps;

/// The shaped card: the caption plus the pair-row and multi-stack child props. A
/// two-way clash flanks the cell (pair row); a rarer 3+-way clash stacks the cell
/// above (multi stack). Exactly one renders; the other guards itself away.
pub(super) struct UnitPositionConflictCardModel {
    pub(super) role_label: String,
    pub(super) pair_row: PositionPairRowProps,
    pub(super) multi_stack: PositionMultiStackProps,
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
        let cell_between = ConflictPositionCellProps {
            collision_column: position_column,
            collision_row: position_row,
            is_top: false,
        };
        let cell_top = ConflictPositionCellProps {
            collision_column: position_column,
            collision_row: position_row,
            is_top: true,
        };
        let (pair, multi) = if abilities.len() == 2 {
            let mut iter = abilities.into_iter();
            let left = iter.next().expect("checked len == 2");
            let right = iter.next().expect("checked len == 2");
            let pair = PositionPair {
                left,
                right,
                cell: cell_between,
            };
            (Some(pair), Vec::new())
        } else {
            (None, abilities)
        };
        let pair_row = PositionPairRowProps { pair };
        let multi_stack = PositionMultiStackProps {
            abilities: multi,
            cell: cell_top,
        };
        Self {
            role_label,
            pair_row,
            multi_stack,
        }
    }
}
