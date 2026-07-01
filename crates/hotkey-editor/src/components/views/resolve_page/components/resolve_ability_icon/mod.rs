pub mod components;
mod props;
mod style;
use crate::assert_component;
use crate::components::views::resolve_page::logic::CarriersDialogData;
use components::resolve_carrier_badge::{ResolveCarrierBadge, ResolveCarrierBadgeProps};
use components::resolve_fight_icon::{ResolveFightIcon, ResolveFightIconProps};
use dioxus::prelude::*;
pub use props::ResolveAbilityIconProps;
use style::CLASS;
assert_component!(ResolveAbilityIcon);

/// One ability icon with a carrier-count badge; clicking opens the carriers dialog.
#[component]
pub fn ResolveAbilityIcon(props: ResolveAbilityIconProps) -> Element {
    let name = props.name;
    let carrier_count = props.carrier_count;
    let carrier_unit_ids = props.carrier_unit_ids;
    let is_winner = props.is_winner;
    let mut carriers_dialog = props.carriers_dialog;
    let has_carriers = !carrier_unit_ids.is_empty();
    let title = format!("{name} — {carrier_count} carriers");
    let icon = props.icon_url.map(|src| ResolveFightIconProps {
        src,
        alt: name.clone(),
    });
    let badge = ResolveCarrierBadgeProps {
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
            if let Some(icon) = icon {
                ResolveFightIcon { ..icon }
            }
            ResolveCarrierBadge { ..badge }
        }
    }
}
