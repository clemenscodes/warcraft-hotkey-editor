mod props;

use crate::components::app::components::shell::components::editor_page::components::shared::toggle_button::{
    ToggleButton, ToggleButtonProps,
};
use dioxus::prelude::*;

use tw_macro::assert_component;

pub use props::CatalogVisibilityButtonProps;

assert_component!(CatalogVisibilityButton);

/// One button of the catalog-visibility toggle. It is the shared [`ToggleButton`]
/// configured for a visibility filter — the only one of the toggles that carries a
/// tooltip.
#[component]
pub fn CatalogVisibilityButton(props: CatalogVisibilityButtonProps) -> Element {
    let button = ToggleButtonProps::from(&props);
    rsx! {
        ToggleButton { ..button }
    }
}
