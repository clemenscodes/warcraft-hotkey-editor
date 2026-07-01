use super::props::ResolveMoveRowProps;
use crate::components::views::resolve_page::components::resolve_ability_icon::ResolveAbilityIconProps;
use crate::components::views::resolve_page::components::resolve_fight_name_btn::ResolveFightNameBtnProps;
use crate::components::views::resolve_page::components::resolve_fight_name_plate::ResolveFightNamePlateProps;
use crate::components::views::resolve_page::components::resolve_move_reason_row::ResolveMoveReasonRowProps;
use crate::components::views::resolve_page::logic::{MiniGridPlacement, ResolveReasonKind};
use dioxus::prelude::*;

/// The rival ability's plate and icon, when the move has one (Fight/Swap).
pub(super) struct AnchorParts {
    pub(super) name_plate: ResolveFightNamePlateProps,
    pub(super) ability: ResolveAbilityIconProps,
}

/// The fully shaped move card: reason badge, mover column, optional rival column,
/// and the from → to placements for the two mini grids.
pub(super) struct ResolveMoveRowModel {
    pub(super) reason_row: ResolveMoveReasonRowProps,
    pub(super) mover_name_btn: ResolveFightNameBtnProps,
    pub(super) mover_ability: ResolveAbilityIconProps,
    pub(super) anchor: Option<AnchorParts>,
    pub(super) from_placements: Vec<MiniGridPlacement>,
    pub(super) to_placements: Vec<MiniGridPlacement>,
}

impl From<&ResolveMoveRowProps> for ResolveMoveRowModel {
    fn from(props: &ResolveMoveRowProps) -> Self {
        let move_view = props.move_view.clone();
        let carriers_dialog = props.carriers_dialog;
        let view_navigation = props.view_navigation;
        let mover = move_view.mover;
        let reason = move_view.reason;
        let mover_unit_id = move_view.mover_unit_id;
        let has_unit = mover_unit_id.is_some();
        let reason_row = ResolveMoveReasonRowProps {
            kind: ResolveReasonKind::from(reason.category),
            label: reason.label.to_owned(),
        };
        let open_unit_id = mover_unit_id.clone();
        let open_mover = EventHandler::new(move |_event: MouseEvent| {
            if let Some(unit_id) = open_unit_id.as_ref() {
                view_navigation.open_unit(unit_id);
            }
        });
        let mover_name_btn = ResolveFightNameBtnProps {
            name: mover.name.clone(),
            object_id: mover.object_id.clone(),
            has_unit,
            onclick: open_mover,
        };
        let mover_ability = ResolveAbilityIconProps {
            name: mover.name.clone(),
            icon_url: mover.icon_url.clone(),
            carrier_count: move_view.mover_carriers,
            carrier_unit_ids: move_view.mover_carrier_unit_ids,
            is_winner: false,
            carriers_dialog,
        };
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
        let anchor = match reason.other_ability {
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
                let name_plate = ResolveFightNamePlateProps {
                    name: anchor_ability.name.clone(),
                    object_id: anchor_ability.object_id.clone(),
                };
                let ability = ResolveAbilityIconProps {
                    name: anchor_ability.name,
                    icon_url: anchor_ability.icon_url,
                    carrier_count: anchor_carriers,
                    carrier_unit_ids: anchor_carrier_unit_ids,
                    is_winner: !is_swap,
                    carriers_dialog,
                };
                let parts = AnchorParts { name_plate, ability };
                Some(parts)
            }
            None => None,
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
