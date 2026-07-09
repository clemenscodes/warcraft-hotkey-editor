use super::components::anchor_column::{AnchorColumnProps, AnchorParts};
use super::components::fight_name_button::FightNameButtonProps;
use super::props::MoveRowProps;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::ability_icon::AbilityIconProps;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::fight_name_plate::FightNamePlateProps;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_reason_row::MoveReasonRowProps;
use crate::components::app::components::shell::components::resolve_page::logic::{MiniGridPlacement, ReasonKind};
use crate::services::carriers::InspectedAbility;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

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
        let mover = move_view.mover();
        let reason = move_view.reason();
        let mover_unit_id = move_view.mover_unit_id();
        let has_unit = mover_unit_id.is_some();
        let reason_category = reason.category();
        let reason_kind = ReasonKind::from(reason_category);
        let reason_label = reason.label().to_owned();
        let reason_row = MoveReasonRowProps {
            kind: reason_kind,
            label: reason_label,
        };
        let open_unit_id = mover_unit_id;
        let open_mover = EventHandler::new(move |_event: MouseEvent| {
            if let Some(unit_id) = open_unit_id {
                view_navigation.open_unit(unit_id);
            }
        });
        let mover_name = mover.name().to_owned();
        let mover_object_id = mover.object_id();
        let mover_icon_url_ref = mover.icon_url();
        let mover_icon_url = mover_icon_url_ref.map(str::to_owned);
        let mover_name_for_button = mover_name.clone();
        let mover_name_btn = FightNameButtonProps {
            name: mover_name_for_button,
            object_id: mover_object_id,
            has_unit,
            onclick: open_mover,
        };
        let mover_carriers = move_view.mover_carriers();
        let mover_carrier_unit_ids_ref = move_view.mover_carrier_unit_ids();
        let mover_carrier_unit_ids = mover_carrier_unit_ids_ref.to_vec();
        let mover_name_for_ability = mover_name.clone();
        let mover_icon_url_for_ability = mover_icon_url.clone();
        let mover_ability = Self::ability(
            mover_name_for_ability,
            mover_icon_url_for_ability,
            mover_carriers,
            mover_carrier_unit_ids,
            false,
        );
        let is_swap = reason.is_swap();
        let anchor_carrier_option = reason.other_carriers();
        let anchor_carriers = anchor_carrier_option.unwrap_or(0);
        let anchor_carrier_unit_ids_ref = reason.other_carrier_unit_ids();
        let anchor_carrier_unit_ids = anchor_carrier_unit_ids_ref.to_vec();
        let from_column = move_view.from_column();
        let from_row = move_view.from_row();
        let to_column = move_view.to_column();
        let to_row = move_view.to_row();
        let mover_icon_url_for_from = mover_icon_url.clone();
        let mover_name_for_from = mover_name.clone();
        let mover_from_placement = MiniGridPlacement::new(
            from_column,
            from_row,
            mover_icon_url_for_from,
            mover_name_for_from,
        );
        let mover_icon_url_for_to = mover_icon_url.clone();
        let mover_name_for_to = mover_name.clone();
        let mover_to_placement =
            MiniGridPlacement::new(to_column, to_row, mover_icon_url_for_to, mover_name_for_to);
        let mut from_placements: Vec<MiniGridPlacement> = vec![mover_from_placement];
        let mut to_placements: Vec<MiniGridPlacement> = vec![mover_to_placement];
        let other_ability_option = reason.other_ability();
        let anchor_parts = match other_ability_option {
            Some(anchor_ability) => {
                let anchor_name = anchor_ability.name().to_owned();
                let anchor_object_id = anchor_ability.object_id();
                let anchor_icon_url_ref = anchor_ability.icon_url();
                let anchor_icon_url = anchor_icon_url_ref.map(str::to_owned);
                let anchor_icon_url_for_after = anchor_icon_url.clone();
                let anchor_name_for_after = anchor_name.clone();
                let anchor_after_placement = MiniGridPlacement::new(
                    from_column,
                    from_row,
                    anchor_icon_url_for_after,
                    anchor_name_for_after,
                );
                to_placements.push(anchor_after_placement);
                if is_swap {
                    let anchor_icon_url_for_before = anchor_icon_url.clone();
                    let anchor_name_for_before = anchor_name.clone();
                    let anchor_before_placement = MiniGridPlacement::new(
                        to_column,
                        to_row,
                        anchor_icon_url_for_before,
                        anchor_name_for_before,
                    );
                    from_placements.push(anchor_before_placement);
                }
                let anchor_name_for_plate = anchor_name.clone();
                let name_plate = FightNamePlateProps {
                    name: anchor_name_for_plate,
                    object_id: anchor_object_id,
                };
                let is_not_swap = !is_swap;
                let ability = Self::ability(
                    anchor_name,
                    anchor_icon_url,
                    anchor_carriers,
                    anchor_carrier_unit_ids,
                    is_not_swap,
                );
                let parts = AnchorParts::new(name_plate, ability);
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
        carrier_unit_ids: Vec<WarcraftObjectId>,
        is_winner: bool,
    ) -> AbilityIconProps {
        let disabled = carrier_unit_ids.is_empty();
        let name_for_inspected = name.clone();
        let inspected = InspectedAbility::new(name_for_inspected, carrier_unit_ids);
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
