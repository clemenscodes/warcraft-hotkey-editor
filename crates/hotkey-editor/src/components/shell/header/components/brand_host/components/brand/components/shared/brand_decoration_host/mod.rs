pub mod components;
mod style;

use crate::assert_component;
use components::brand_decoration::BrandDecoration;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(BrandDecorationHost);

/// The decoration's sizing container: it establishes the container-query context and
/// owns the flourish's box per band, so the leaf scales to fill it. Brand decides how
/// large the flourish is drawn here; the leaf itself carries no fixed dimensions.
#[component]
pub fn BrandDecorationHost() -> Element {
    rsx! {
        div {
            class: CLASS,
            BrandDecoration {}
        }
    }
}
