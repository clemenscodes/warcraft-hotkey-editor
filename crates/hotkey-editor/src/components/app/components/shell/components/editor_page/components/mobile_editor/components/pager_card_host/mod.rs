pub mod components;
mod model;
mod view;

pub use view::PagerCardHostView;
mod style;

use components::pager_card::PagerCard;
use dioxus::prelude::*;
use model::PagerCardHostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PagerCardHost(props: PagerCardHostModel) -> Element {
    let unit_id = props.unit_id;
    rsx! {
        div {
            class: CLASS,
            PagerCard {
                unit_id,
            }
        }
    }
}

assert_component!(PagerCardHost);
