pub mod components;
mod props;
mod style;

use components::category_track::{CategoryTrack, CategoryTrackProps};
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

pub use props::CategoryScrollProps;

assert_component!(CategoryScroll);

/// The scroll region around the category-section track: a vertical, gold-scrollbar
/// column on the sidebar (the scrollbar is revealed by the list group's hover), a
/// horizontal snap carousel on small screens.
#[component]
pub fn CategoryScroll(props: CategoryScrollProps) -> Element {
    let track = CategoryTrackProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            CategoryTrack { ..track }
        }
    }
}
