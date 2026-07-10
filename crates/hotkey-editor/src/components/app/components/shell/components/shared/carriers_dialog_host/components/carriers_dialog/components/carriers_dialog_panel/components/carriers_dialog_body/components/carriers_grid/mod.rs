pub mod components;
mod props;
mod style;

use components::carrier_card::CarrierCard;
use dioxus::prelude::*;
use props::CarriersGridProps;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CarriersGrid(props: CarriersGridProps) -> Element {
    let carriers = props.carriers;
    rsx! {
        div {
            class: CLASS,
            for carrier in carriers {
                CarrierCard { carrier }
            }
        }
    }
}

assert_component!(CarriersGrid);
