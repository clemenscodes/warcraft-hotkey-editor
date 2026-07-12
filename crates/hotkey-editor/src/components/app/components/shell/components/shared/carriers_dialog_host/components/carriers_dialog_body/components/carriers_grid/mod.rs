pub mod components;
mod model;
mod view;

pub use view::CarriersGridView;
mod style;

use components::carrier_card::CarrierCard;
use dioxus::prelude::*;
use model::CarriersGridModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CarriersGrid(props: CarriersGridModel) -> Element {
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
