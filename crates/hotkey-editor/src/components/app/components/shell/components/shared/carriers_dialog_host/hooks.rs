use super::components::carriers_dialog::CarriersDialogProps;
use crate::services::carriers::{self, InspectedAbility};
use dioxus::prelude::*;

/// The host's seam: read the ability the trigger opened (if any), resolve its carriers
/// through the query, and shape the dialog's props — or `None` when nothing is open. The
/// cards read the navigation they deep-link through from context themselves.
pub(super) fn use_carriers_dialog_host(
    open_state: Signal<Option<InspectedAbility>>,
) -> Option<CarriersDialogProps> {
    let current = open_state.read().clone();
    let ability = current?;
    let title = ability.ability_name().to_owned();
    let carriers = carriers::for_unit_ids(ability.carrier_unit_ids());
    let props = CarriersDialogProps {
        title,
        carriers,
        open_state,
    };
    Some(props)
}
