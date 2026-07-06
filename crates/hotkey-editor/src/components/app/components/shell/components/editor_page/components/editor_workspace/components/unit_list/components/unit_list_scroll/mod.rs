pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use components::unit_list_track::UnitListTrack;
use style::CLASS;
use tw_macro::assert_component;

pub use props::UnitListScrollProps;

assert_component!(UnitListScroll);

/// The scrolling region of the unit list, holding the track of category sections.
#[component]
pub fn UnitListScroll(props: UnitListScrollProps) -> Element {
    let children = props.children;
    rsx! {
        div { class: CLASS,
            UnitListTrack { {children} }
        }
    }
}
