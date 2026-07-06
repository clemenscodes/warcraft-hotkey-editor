use super::components::carriers_dialog::CarriersDialogProps;
use crate::services::carriers::{Carriers, InspectedAbility};
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;

/// The host's seam: read the ability the trigger opened (if any), resolve its carriers
/// through the query, and shape the dialog's props — or `None` when nothing is open.
pub(super) fn use_carriers_dialog_host(
    open_state: Signal<Option<InspectedAbility>>,
) -> Option<CarriersDialogProps> {
    let view_navigation = use_view_navigation();
    let current = open_state.read().clone();
    let ability = current?;
    let title = ability.ability_name().to_owned();
    let carriers = Carriers::for_unit_ids(ability.carrier_unit_ids());
    let props = CarriersDialogProps {
        title,
        carriers,
        view_navigation,
        open_state,
    };
    Some(props)
}
