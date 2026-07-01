pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use components::catalog_visibility_button::CatalogVisibilityButton;
use style::CLASS;

pub use props::CatalogVisibilityToggleProps;

assert_component!(CatalogVisibilityToggle);

/// The No-abilities / All-variants toggle that widens which units the list shows.
#[component]
pub fn CatalogVisibilityToggle(props: CatalogVisibilityToggleProps) -> Element {
    let mut show_abilityless_units = props.show_abilityless_units;
    let mut expand_variants = props.expand_variants;
    let show_abilityless_active = *show_abilityless_units.read();
    let expand_variants_active = *expand_variants.read();
    let toggle_abilityless = EventHandler::new(move |_event: MouseEvent| {
        show_abilityless_units.set(!show_abilityless_active);
    });
    let toggle_variants = EventHandler::new(move |_event: MouseEvent| {
        expand_variants.set(!expand_variants_active);
    });
    rsx! {
        div {
            class: CLASS,
            role: "group",
            aria_label: "Catalog visibility",
            CatalogVisibilityButton {
                label: "No abilities",
                title: "Show units without abilities (for stats)",
                is_active: show_abilityless_active,
                on_toggle: toggle_abilityless,
            }
            CatalogVisibilityButton {
                label: "All variants",
                title: "List every tier / upgrade variant separately",
                is_active: expand_variants_active,
                on_toggle: toggle_variants,
            }
        }
    }
}
