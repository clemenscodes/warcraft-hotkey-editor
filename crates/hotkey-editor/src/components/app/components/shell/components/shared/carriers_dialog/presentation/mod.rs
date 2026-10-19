use super::model::CarriersDialogModel;
use crate::services::carriers::{self, CarrierUnitView};
use dioxus::prelude::*;

pub(super) struct OpenCarriersDialog {
    pub(super) title: String,
    pub(super) carriers: Vec<CarrierUnitView>,
    pub(super) on_open_change: Callback<bool>,
}

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
