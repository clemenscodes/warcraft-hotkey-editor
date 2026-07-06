use super::data::STUCK_LABEL;
use super::props::UnresolvedRowProps;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::ability_icon::AbilityIconProps;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::fight_name_plate::FightNamePlateProps;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_reason_row::MoveReasonRowProps;
use crate::components::app::components::shell::components::resolve_page::logic::{MiniGridPlacement, ReasonKind};
use crate::services::carriers::InspectedAbility;

/// The stuck card's reason badge, plate, icon, and the single cell it is stuck on.
pub(super) struct UnresolvedRowModel {
    pub(super) reason_row: MoveReasonRowProps,
    pub(super) name_plate: FightNamePlateProps,
    pub(super) ability: AbilityIconProps,
    pub(super) placements: Vec<MiniGridPlacement>,
}

impl From<&UnresolvedRowProps> for UnresolvedRowModel {
    fn from(props: &UnresolvedRowProps) -> Self {
        let view = props.unresolved_view.clone();
        let reason_row = MoveReasonRowProps {
            kind: ReasonKind::Stuck,
            label: STUCK_LABEL.to_owned(),
        };
        let ability = view.ability;
        let name_plate = FightNamePlateProps {
            name: ability.name.clone(),
            object_id: ability.object_id.clone(),
        };
        let placement = MiniGridPlacement {
            column: view.column,
            row: view.row,
            icon_url: ability.icon_url.clone(),
            name: ability.name.clone(),
        };
        let placements: Vec<MiniGridPlacement> = vec![placement];
        let carrier_unit_ids = view.carrier_unit_ids;
        let disabled = carrier_unit_ids.is_empty();
        let inspected = InspectedAbility::new(ability.name.clone(), carrier_unit_ids);
        let icon = AbilityIconProps {
            name: ability.name,
            icon_url: ability.icon_url,
            carrier_count: view.carrier_count,
            is_winner: false,
            disabled,
            inspected,
        };
        Self {
            reason_row,
            name_plate,
            ability: icon,
            placements,
        }
    }
}
