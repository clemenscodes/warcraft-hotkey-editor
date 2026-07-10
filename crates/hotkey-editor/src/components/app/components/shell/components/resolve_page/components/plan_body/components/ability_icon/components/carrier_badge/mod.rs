pub mod components;
mod props;

use components::regular_carrier_badge::RegularCarrierBadge;
use components::winner_carrier_badge::WinnerCarrierBadge;
use dioxus::prelude::*;
use props::CarrierBadgeProps;
use tw_macro::assert_component;

/// The carrier-count badge on an ability icon. A dispatcher: from whether its ability
/// wins the cell it renders the gold `WinnerCarrierBadge` xor the muted
/// `RegularCarrierBadge`; there is no `data-win` attribute.
#[component]
pub fn CarrierBadge(props: CarrierBadgeProps) -> Element {
    let count = props.count;
    match props.is_winner {
        true => rsx! {
            WinnerCarrierBadge { count }
        },
        false => rsx! {
            RegularCarrierBadge { count }
        },
    }
}

assert_component!(CarrierBadge);
