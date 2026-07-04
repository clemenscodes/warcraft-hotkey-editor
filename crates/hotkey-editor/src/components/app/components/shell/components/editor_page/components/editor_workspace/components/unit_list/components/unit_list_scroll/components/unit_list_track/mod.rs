mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::UnitListTrackProps;

assert_component!(UnitListTrack);

/// The inner track that lays out the category sections.
#[component]
pub fn UnitListTrack(props: UnitListTrackProps) -> Element {
    let children = props.children;
    rsx! {
        div { class: CLASS, {children} }
    }
}
