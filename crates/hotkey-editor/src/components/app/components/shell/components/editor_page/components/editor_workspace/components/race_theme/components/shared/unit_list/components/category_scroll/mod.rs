pub mod components;
mod props;
mod style;

use components::category_track::CategoryTrack;
use dioxus::prelude::*;
use props::CategoryScrollProps;
use style::CLASS;
use tw_macro::assert_component;

/// The scroll region around the category-section track: a vertical, gold-scrollbar
/// column on the sidebar (the scrollbar is revealed by the list group's hover), a
/// horizontal snap carousel on small screens.
#[component]
pub fn CategoryScroll(props: CategoryScrollProps) -> Element {
    let sections = props.sections;
    rsx! {
        div {
            class: CLASS,
            CategoryTrack { sections }
        }
    }
}

assert_component!(CategoryScroll);
