pub mod components;
mod props;
mod style;

use components::carrier_card::CarrierCard;
use dioxus::prelude::*;
pub use props::CarriersGridProps;
use style::CLASS;
use tw_macro::assert_component;

assert_component!(CarriersGrid);

#[component]
pub fn CarriersGrid(props: CarriersGridProps) -> Element {
    let cards = props.cards;
    rsx! {
        div {
            class: CLASS,
            for card in cards {
                CarrierCard { ..card }
            }
        }
    }
}
