pub mod components;
mod style;

use components::brand_host::BrandHost;
use components::grid_layout_editor_button_host::GridLayoutEditorButtonHost;
use components::toolbar::Toolbar;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            class: CLASS,
            BrandHost {}
            GridLayoutEditorButtonHost {}
            Toolbar {}
        }
    }
}

assert_component!(Header);
