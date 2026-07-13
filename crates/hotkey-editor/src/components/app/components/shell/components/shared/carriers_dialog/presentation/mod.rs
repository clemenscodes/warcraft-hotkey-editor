use super::model::CarriersDialogModel;
use crate::services::carriers::{self, CarrierUnitView};
use dioxus::prelude::*;

/// The open carriers dialog's shaped data: the ability's title, its resolved carriers, and
/// the change handler mirroring the headless dialog's own close (escape, outside click)
/// back to the trigger's open state, which it clears.
pub(super) struct OpenCarriersDialog {
    pub(super) title: String,
    pub(super) carriers: Vec<CarrierUnitView>,
    pub(super) on_open_change: Callback<bool>,
}

/// The host's seam: read the ability the trigger opened (if any), resolve its carriers
/// through the query, and shape the open dialog — or `None` when nothing is open. Body
/// scroll is locked once by `WarcraftDialog`. The cards read the navigation they deep-link
/// through from context themselves.
pub(super) fn use_carriers_dialog(props: &CarriersDialogModel) -> Option<OpenCarriersDialog> {
    let ability = props.ability.clone()?;
    let title = ability.ability_name().to_owned();
    let carrier_unit_ids = ability.carrier_unit_ids();
    let carriers = carriers::for_unit_ids(carrier_unit_ids);
    let on_close = props.on_close;
    let on_open_change = Callback::new(move |is_open: bool| {
        if !is_open {
            on_close.call(());
        }
    });
    let dialog = OpenCarriersDialog {
        title,
        carriers,
        on_open_change,
    };
    Some(dialog)
}
