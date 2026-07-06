pub mod components;
mod style;

use components::brand::Brand;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(BrandHost);

/// The brand's sizing container: it establishes the container-query context and is
/// sized responsively per viewport, so the wordmark scales as one drawing. The title,
/// gap, and both flourishes express their lengths in `cqi` against this box — shrink
/// the box and the whole brand scales down linearly, like a single SVG, so the title
/// never truncates.
#[component]
pub fn BrandHost() -> Element {
    rsx! {
        div {
            class: CLASS,
            Brand {}
        }
    }
}
