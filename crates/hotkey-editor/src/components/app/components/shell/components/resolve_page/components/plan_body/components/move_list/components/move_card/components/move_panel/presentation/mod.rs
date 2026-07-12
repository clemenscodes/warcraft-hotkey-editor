use crate::components::app::components::shell::components::resolve_page::presentation::{
    MiniGridPlacement, MoveView, ReasonKind,
};

/// The move card's shaped body, derived from the move: the reason badge's kind and
/// label, and the "before"/"after" mini-grid placements (the mover on both sides, plus
/// the rival landing on the mover's old cell — and, on a swap, the rival's old cell).
pub(super) struct MovePanelPresentation {
    pub(super) reason_kind: ReasonKind,
    pub(super) reason_label: String,
    pub(super) from_placements: Vec<MiniGridPlacement>,
    pub(super) to_placements: Vec<MiniGridPlacement>,
}

impl From<&MoveView> for MovePanelPresentation {
    fn from(move_view: &MoveView) -> Self {
        let mover = move_view.mover();
        let reason = move_view.reason();
        let reason_category = reason.category();
        let reason_kind = ReasonKind::from(reason_category);
        let reason_label = reason.label().to_owned();
        let is_swap = reason.is_swap();
        let from = move_view.from();
        let to = move_view.to();
        let mover_name = mover.name().to_owned();
        let mover_icon_url = mover.icon_url().map(str::to_owned);
        let mover_icon_url_for_from = mover_icon_url.clone();
        let mover_name_for_from = mover_name.clone();
        let mover_from_placement =
            MiniGridPlacement::new(from, mover_icon_url_for_from, mover_name_for_from);
        let mover_icon_url_for_to = mover_icon_url.clone();
        let mover_name_for_to = mover_name.clone();
        let mover_to_placement =
            MiniGridPlacement::new(to, mover_icon_url_for_to, mover_name_for_to);
        let mut from_placements: Vec<MiniGridPlacement> = vec![mover_from_placement];
        let mut to_placements: Vec<MiniGridPlacement> = vec![mover_to_placement];
        let other_ability_option = reason.other_ability();
        if let Some(anchor_ability) = other_ability_option {
            let anchor_name = anchor_ability.name().to_owned();
            let anchor_icon_url_ref = anchor_ability.icon_url();
            let anchor_icon_url = anchor_icon_url_ref.map(str::to_owned);
            let anchor_icon_url_for_after = anchor_icon_url.clone();
            let anchor_name_for_after = anchor_name.clone();
            let anchor_after_placement =
                MiniGridPlacement::new(from, anchor_icon_url_for_after, anchor_name_for_after);
            to_placements.push(anchor_after_placement);
            if is_swap {
                let anchor_before_placement =
                    MiniGridPlacement::new(to, anchor_icon_url, anchor_name);
                from_placements.push(anchor_before_placement);
            }
        }
        Self {
            reason_kind,
            reason_label,
            from_placements,
            to_placements,
        }
    }
}
