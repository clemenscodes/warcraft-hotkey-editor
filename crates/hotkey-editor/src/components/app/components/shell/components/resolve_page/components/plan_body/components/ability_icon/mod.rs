pub mod components;
mod props;
mod style;
use crate::components::app::components::shell::components::resolve_page::logic::CarriersDialogData;
use components::carrier_badge::{CarrierBadge, CarrierBadgeProps};
use components::fight_icon::{FightIcon, FightIconProps};
use dioxus::prelude::*;
pub use props::AbilityIconProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(AbilityIcon);

/// One ability icon with a carrier-count badge; clicking opens the carriers dialog.
#[component]
pub fn AbilityIcon(props: AbilityIconProps) -> Element {
    let name = props.name;
    let carrier_count = props.carrier_count;
    let carrier_unit_ids = props.carrier_unit_ids;
    let is_winner = props.is_winner;
    let mut carriers_dialog = props.carriers_dialog;
    let has_carriers = !carrier_unit_ids.is_empty();
    let title = format!("{name} — {carrier_count} carriers");
    let icon = FightIconProps {
        src: props.icon_url,
        alt: name.clone(),
    };
    let badge = CarrierBadgeProps {
        count: carrier_count,
        is_winner,
    };
    let dialog_name = name;
    let onclick = move |_event: MouseEvent| {
        if carrier_unit_ids.is_empty() {
            return;
        }
        let data = CarriersDialogData::new(dialog_name.clone(), &carrier_unit_ids);
        carriers_dialog.set(Some(data));
    };
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            disabled: !has_carriers,
            title,
            onclick,
            FightIcon { ..icon }
            CarrierBadge { ..badge }
        }
    }
}
