pub mod components;
mod model;
mod view;

pub use view::PagerTrackView;
mod style;

use components::pager_page::PagerPage;
use dioxus::prelude::*;
use model::PagerTrackModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PagerTrack(props: PagerTrackModel) -> Element {
    let pages = props.pages;
    rsx! {
        div {
            class: CLASS,
            for page in pages {
                PagerPage {
                    key: "{page.unit_ids.first().map(|unit_id| unit_id.to_string()).unwrap_or_default()}",
                    unit_ids: page.unit_ids.clone(),
                }
            }
        }
    }
}

assert_component!(PagerTrack);
