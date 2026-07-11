use crate::services::carriers::{self, CarrierUnitView, InspectedAbility};
use dioxus::prelude::*;

/// The open carriers dialog's shaped data: the ability's title, its resolved carriers, and
/// the trigger's open-state signal the dialog clears when closed.
pub(super) struct OpenCarriersDialog {
    pub(super) title: String,
    pub(super) carriers: Vec<CarrierUnitView>,
    pub(super) open_state: Signal<Option<InspectedAbility>>,
}

/// The host's seam: read the ability the trigger opened (if any), resolve its carriers
/// through the query, and shape the open dialog — or `None` when nothing is open. The
/// cards read the navigation they deep-link through from context themselves.
pub(super) fn use_carriers_dialog_host(
    open_state: Signal<Option<InspectedAbility>>,
) -> Option<OpenCarriersDialog> {
    let current = open_state.read().clone();
    let ability = current?;
    let title = ability.ability_name().to_owned();
    let carrier_unit_ids = ability.carrier_unit_ids();
    let carriers = carriers::for_unit_ids(carrier_unit_ids);
    let dialog = OpenCarriersDialog {
        title,
        carriers,
        open_state,
    };
    Some(dialog)
}
