use crate::components::app::components::shell::components::resolve_page::presentation::MoveView;
use crate::services::carriers::InspectedAbility;
use warcraft_api::WarcraftObjectId;

pub(super) struct AnchorColumnPresentation {
    pub(super) name: String,
    pub(super) object_id: WarcraftObjectId,
    pub(super) icon_url: Option<String>,
    pub(super) carrier_count: usize,
    pub(super) is_winner: bool,
    pub(super) disabled: bool,
    pub(super) inspected: InspectedAbility,
}

impl AnchorColumnPresentation {
    pub(super) fn for_move(move_view: &MoveView) -> Option<Self> {
        let reason = move_view.reason();
        let anchor_ability = reason.other_ability()?;
        let is_swap = reason.is_swap();
        let is_winner = !is_swap;
        let name = anchor_ability.name().to_owned();
        let object_id = anchor_ability.object_id();
        let icon_url = anchor_ability.icon_url().map(str::to_owned);
        let carrier_count = reason.other_carriers().unwrap_or(0);
        let carrier_unit_ids_ref = reason.other_carrier_unit_ids();
        let carrier_unit_ids = carrier_unit_ids_ref.to_vec();
        let disabled = carrier_unit_ids.is_empty();
        let name_for_inspected = name.clone();
        let inspected = InspectedAbility::new(name_for_inspected, carrier_unit_ids);
        let model = Self {
            name,
            object_id,
            icon_url,
            carrier_count,
            is_winner,
            disabled,
            inspected,
        };
        Some(model)
    }
}
