pub mod components;
mod model;
mod view;

pub use view::PagerCardHeaderView;
mod style;

use components::pager_card_id::PagerCardId;
use components::pager_card_name::PagerCardName;
use components::pager_card_portrait::PagerCardPortrait;
use dioxus::prelude::*;
use model::PagerCardHeaderModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PagerCardHeader(props: PagerCardHeaderModel) -> Element {
    let icon_url = props.icon_url;
    let name = props.name;
    let unit_id = props.unit_id;
    rsx! {
        div {
            class: CLASS,
            PagerCardPortrait {
                src: icon_url,
            }
            PagerCardName {
                name,
            }
            PagerCardId {
                unit_id,
            }
        }
    }
}

assert_component!(PagerCardHeader);
