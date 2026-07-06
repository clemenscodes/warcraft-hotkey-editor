use super::components::anchor_column::{AnchorColumnProps, AnchorParts};
use super::components::fight_name_button::FightNameButtonProps;
use super::props::MoveRowProps;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::ability_icon::AbilityIconProps;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::fight_name_plate::FightNamePlateProps;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_reason_row::MoveReasonRowProps;
use crate::components::app::components::shell::components::resolve_page::logic::{MiniGridPlacement, ReasonKind};
use crate::services::carriers::InspectedAbility;
use dioxus::prelude::*;

/// The fully shaped move card: reason badge, mover column, rival column props (the
/// rival is optional; the column renders itself away when absent), and the from →
/// to placements for the two mini grids.
pub(super) struct MoveRowModel {
    pub(super) reason_row: MoveReasonRowProps,
    pub(super) mover_name_btn: FightNameButtonProps,
    pub(super) mover_ability: AbilityIconProps,
    pub(super) anchor: AnchorColumnProps,
    pub(super) from_placements: Vec<MiniGridPlacement>,
    pub(super) to_placements: Vec<MiniGridPlacement>,
}

impl From<&MoveRowProps> for MoveRowModel {
    fn from(props: &MoveRowProps) -> Self {
        let move_view = props.move_view.clone();
        let view_navigation = props.view_navigation;
        let mover = move_view.mover;
        let reason = move_view.reason;
        let mover_unit_id = move_view.mover_unit_id;
        let has_unit = mover_unit_id.is_some();
        let reason_row = MoveReasonRowProps {
            kind: ReasonKind::from(reason.category),
            label: reason.label.to_owned(),
        };
        let open_unit_id = mover_unit_id.clone();
        let open_mover = EventHandler::new(move |_event: MouseEvent| {
            if let Some(unit_id) = open_unit_id.as_ref() {
                view_navigation.open_unit(unit_id);
            }
        });
        let mover_name_btn = FightNameButtonProps {
            name: mover.name.clone(),
            object_id: mover.object_id.clone(),
            has_unit,
            onclick: open_mover,
        };
        let mover_ability = Self::ability(
            mover.name.clone(),
            mover.icon_url.clone(),
            move_view.mover_carriers,
            move_view.mover_carrier_unit_ids,
            false,
        );
        let is_swap = reason.is_swap;
        let anchor_carriers = reason.other_carriers.unwrap_or(0);
        let anchor_carrier_unit_ids = reason.other_carrier_unit_ids;
        let from_column = move_view.from_column;
        let from_row = move_view.from_row;
        let to_column = move_view.to_column;
        let to_row = move_view.to_row;
        let mover_from_placement = MiniGridPlacement {
            column: from_column,
            row: from_row,
            icon_url: mover.icon_url.clone(),
            name: mover.name.clone(),
        };
        let mover_to_placement = MiniGridPlacement {
            column: to_column,
            row: to_row,
            icon_url: mover.icon_url.clone(),
            name: mover.name.clone(),
        };
        let mut from_placements: Vec<MiniGridPlacement> = vec![mover_from_placement];
        let mut to_placements: Vec<MiniGridPlacement> = vec![mover_to_placement];
        let anchor_parts = match reason.other_ability {
            Some(anchor_ability) => {
                let anchor_after_placement = MiniGridPlacement {
                    column: from_column,
                    row: from_row,
                    icon_url: anchor_ability.icon_url.clone(),
                    name: anchor_ability.name.clone(),
                };
                to_placements.push(anchor_after_placement);
                if is_swap {
                    let anchor_before_placement = MiniGridPlacement {
                        column: to_column,
                        row: to_row,
                        icon_url: anchor_ability.icon_url.clone(),
                        name: anchor_ability.name.clone(),
                    };
                    from_placements.push(anchor_before_placement);
                }
                let name_plate = FightNamePlateProps {
                    name: anchor_ability.name.clone(),
                    object_id: anchor_ability.object_id.clone(),
                };
                let ability = Self::ability(
                    anchor_ability.name,
                    anchor_ability.icon_url,
                    anchor_carriers,
                    anchor_carrier_unit_ids,
                    !is_swap,
                );
                let parts = AnchorParts {
                    name_plate,
                    ability,
                };
                Some(parts)
            }
            None => None,
        };
        let anchor = AnchorColumnProps {
            anchor: anchor_parts,
        };
        Self {
            reason_row,
            mover_name_btn,
            mover_ability,
            anchor,
            from_placements,
            to_placements,
        }
    }
}

impl MoveRowModel {
    fn ability(
        name: String,
        icon_url: Option<String>,
        carrier_count: usize,
        carrier_unit_ids: Vec<String>,
        is_winner: bool,
    ) -> AbilityIconProps {
        let disabled = carrier_unit_ids.is_empty();
        let inspected = InspectedAbility::new(name.clone(), carrier_unit_ids);
        AbilityIconProps {
            name,
            icon_url,
            carrier_count,
            is_winner,
            disabled,
            inspected,
        }
    }
}
