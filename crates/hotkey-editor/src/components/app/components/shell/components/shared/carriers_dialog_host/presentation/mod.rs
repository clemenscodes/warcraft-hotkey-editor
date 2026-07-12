use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use crate::services::carriers::{self, CarrierUnitView, InspectedAbility};
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
/// through the query, and shape the open dialog — or `None` when nothing is open. It also
/// locks body scroll while an ability is open (the mount is conditional here, so the lock
/// tracks the open state through a mirrored signal). The cards read the navigation they
/// deep-link through from context themselves.
pub(super) fn use_carriers_dialog_host(
    open_state: Signal<Option<InspectedAbility>>,
) -> Option<OpenCarriersDialog> {
    let mut open_flag = use_signal(|| false);
    use_effect(move || {
        let is_open = open_state.read().is_some();
        open_flag.set(is_open);
    });
    use_body_scroll_lock(open_flag);
    let current = open_state.read().clone();
    let ability = current?;
    let title = ability.ability_name().to_owned();
    let carrier_unit_ids = ability.carrier_unit_ids();
    let carriers = carriers::for_unit_ids(carrier_unit_ids);
    let mut clear_state = open_state;
    let on_open_change = Callback::new(move |is_open: bool| {
        if !is_open {
            clear_state.set(None);
        }
    });
    let dialog = OpenCarriersDialog {
        title,
        carriers,
        on_open_change,
    };
    Some(dialog)
}
