pub mod components;
mod logic;
mod props;
mod style;

use dioxus::prelude::*;

use components::catalog_visibility_button::CatalogVisibilityButton;
use logic::CatalogVisibilityToggleModel;
use style::CLASS;
use tw_macro::assert_component;

pub use props::CatalogVisibilityToggleProps;

assert_component!(CatalogVisibilityToggle);

/// The No-abilities / All-variants toggle that widens which units the list shows.
#[component]
pub fn CatalogVisibilityToggle(props: CatalogVisibilityToggleProps) -> Element {
    let CatalogVisibilityToggleModel {
        abilityless_button,
        variants_button,
    } = CatalogVisibilityToggleModel::from(&props);
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
