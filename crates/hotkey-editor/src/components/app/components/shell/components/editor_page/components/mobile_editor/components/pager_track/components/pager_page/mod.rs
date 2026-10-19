pub mod components;
mod model;
mod view;

pub use view::PagerPageView;
mod style;

use components::pager_card_host::PagerCardHost;
use dioxus::prelude::*;
use model::PagerPageModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PagerPage(props: PagerPageModel) -> Element {
    let unit_ids = props.unit_ids;
    rsx! {
        div {
            class: CLASS,
            for unit_id in unit_ids {
                PagerCardHost {
                    key: "{unit_id}",
                    unit_id,
                }
            }
        }
    }
}

assert_component!(PagerPage);
