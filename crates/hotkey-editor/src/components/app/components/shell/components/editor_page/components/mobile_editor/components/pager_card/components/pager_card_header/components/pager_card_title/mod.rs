pub mod components;
mod model;
mod view;

pub use view::PagerCardTitleView;
mod style;

use components::pager_card_id::PagerCardId;
use components::pager_card_name::PagerCardName;
use dioxus::prelude::*;
use model::PagerCardTitleModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PagerCardTitle(props: PagerCardTitleModel) -> Element {
    let name = props.name;
    let unit_id = props.unit_id;
    rsx! {
        div {
            class: CLASS,
            PagerCardName {
                name,
            }
            PagerCardId {
                unit_id,
            }
        }
    }
}

assert_component!(PagerCardTitle);
