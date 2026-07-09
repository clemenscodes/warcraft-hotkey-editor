pub mod components;
mod logic;
mod props;

use components::regular_carrier_badge::{RegularCarrierBadge, RegularCarrierBadgeProps};
use components::winner_carrier_badge::{WinnerCarrierBadge, WinnerCarrierBadgeProps};
use dioxus::prelude::*;
pub use props::CarrierBadgeProps;
use tw_macro::assert_component;
assert_component!(CarrierBadge);

/// The carrier-count badge on an ability icon. A dispatcher: from whether its ability
/// wins the cell it renders the gold `WinnerCarrierBadge` xor the muted
/// `RegularCarrierBadge`; there is no `data-win` attribute.
#[component]
pub fn CarrierBadge(props: CarrierBadgeProps) -> Element {
    match props.is_winner {
        true => {
            let badge = WinnerCarrierBadgeProps::from(&props);
            rsx! {
                WinnerCarrierBadge { ..badge }
            }
        }
        false => {
            let badge = RegularCarrierBadgeProps::from(&props);
            rsx! {
                RegularCarrierBadge { ..badge }
            }
        }
    }
}
