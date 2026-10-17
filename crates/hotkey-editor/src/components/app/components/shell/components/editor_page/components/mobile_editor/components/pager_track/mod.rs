pub mod components;
mod model;
mod view;

pub use view::PagerTrackView;
mod style;

use components::pager_card_host::PagerCardHost;
use dioxus::prelude::*;
use model::PagerTrackModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PagerTrack(props: PagerTrackModel) -> Element {
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

assert_component!(PagerTrack);
