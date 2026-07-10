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
        let stuck_label = STUCK_LABEL.to_owned();
        let reason_row = MoveReasonRowProps {
            kind: ReasonKind::Stuck,
            label: stuck_label,
        };
        let ability_display = view.ability();
        let ability_name = ability_display.name().to_owned();
        let ability_object_id = ability_display.object_id();
        let ability_icon_url_ref = ability_display.icon_url();
        let ability_icon_url = ability_icon_url_ref.map(str::to_owned);
        let ability_name_for_plate = ability_name.clone();
        let name_plate = FightNamePlateProps {
            name: ability_name_for_plate,
            object_id: ability_object_id,
        };
        let position = view.position();
        let ability_icon_url_for_placement = ability_icon_url.clone();
        let ability_name_for_placement = ability_name.clone();
        let placement = MiniGridPlacement::new(
            position,
            ability_icon_url_for_placement,
            ability_name_for_placement,
        );
        let placements: Vec<MiniGridPlacement> = vec![placement];
        let carrier_unit_ids_ref = view.carrier_unit_ids();
        let carrier_unit_ids = carrier_unit_ids_ref.to_vec();
        let disabled = carrier_unit_ids.is_empty();
        let ability_name_for_inspected = ability_name.clone();
        let inspected = InspectedAbility::new(ability_name_for_inspected, carrier_unit_ids);
        let carrier_count = view.carrier_count();
        let icon = AbilityIconProps {
            name: ability_name,
            icon_url: ability_icon_url,
            carrier_count,
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
