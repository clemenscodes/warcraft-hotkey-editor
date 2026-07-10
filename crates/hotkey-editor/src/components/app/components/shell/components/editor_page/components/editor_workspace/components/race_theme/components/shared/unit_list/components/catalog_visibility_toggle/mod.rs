pub mod components;
mod logic;
mod style;

use crate::services::editor_state::context::use_editor_state;
use components::catalog_visibility_button::CatalogVisibilityButton;
use dioxus::prelude::*;
use logic::{CatalogVisibilitySignals, CatalogVisibilityToggleModel};
use style::CLASS;
use tw_macro::assert_component;

assert_component!(CatalogVisibilityToggle);

/// The No-abilities / All-variants toggle that widens which units the list shows. It
/// reads and flips the two visibility signals from editor context, so it needs no
/// props: each button is built from the context signals directly.
#[component]
pub fn CatalogVisibilityToggle() -> Element {
    let editor = use_editor_state();
    let signals = CatalogVisibilitySignals {
        show_abilityless_units: editor.show_abilityless_units(),
        expand_variants: editor.expand_variants(),
    };
    let CatalogVisibilityToggleModel {
        abilityless_button,
        variants_button,
    } = CatalogVisibilityToggleModel::from(signals);
    rsx! {
        div {
            class: CLASS,
            role: "group",
            aria_label: "Catalog visibility",
            CatalogVisibilityButton { ..abilityless_button }
            CatalogVisibilityButton { ..variants_button }
        }
    }
}
