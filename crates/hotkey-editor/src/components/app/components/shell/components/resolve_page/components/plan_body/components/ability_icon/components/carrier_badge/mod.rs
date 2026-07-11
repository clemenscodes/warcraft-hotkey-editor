pub mod components;
mod model;
mod view;

pub use view::CarrierBadgeView;

use components::regular_carrier_badge::RegularCarrierBadge;
use components::winner_carrier_badge::WinnerCarrierBadge;
use dioxus::prelude::*;
use model::CarrierBadgeModel;
use tw_macro::assert_component;

/// The carrier-count badge on an ability icon. A dispatcher: from whether its ability
/// wins the cell it renders the gold `WinnerCarrierBadge` xor the muted
/// `RegularCarrierBadge`; there is no `data-win` attribute.
#[component]
pub fn CarrierBadge(props: CarrierBadgeModel) -> Element {
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
