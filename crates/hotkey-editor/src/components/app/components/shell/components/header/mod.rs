pub mod components;
mod style;

use crate::assert_component;
use components::brand_host::BrandHost;
use components::grid_layout_button_host::GridLayoutButtonHost;
use components::toolbar::Toolbar;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(Header);

/// The app's top chrome: the brand on the left, the global grid-layout button
/// centered, and the toolbar on the right. Below 1280px the centered button hides
/// and the layout collapses to brand-left, toolbar-right. Pure layout — it wires
/// nothing and threads nothing; every child sources its own state.
#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            class: CLASS,
            BrandHost {}
            GridLayoutButtonHost {}
            Toolbar {}
        }
    }
}
