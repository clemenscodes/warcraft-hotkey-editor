use super::components::catalog_visibility_button::CatalogVisibilityButtonProps;
use super::props::CatalogVisibilityToggleProps;
use dioxus::prelude::*;

/// The two catalog-visibility options, each finished with its on/off state and
/// flip handler.
pub(super) struct CatalogVisibilityToggleModel {
    pub(super) abilityless_button: CatalogVisibilityButtonProps,
    pub(super) variants_button: CatalogVisibilityButtonProps,
}

impl From<&CatalogVisibilityToggleProps> for CatalogVisibilityToggleModel {
    fn from(props: &CatalogVisibilityToggleProps) -> Self {
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
        let abilityless_button = CatalogVisibilityButtonProps {
            label: "No abilities",
            title: "Show units without abilities (for stats)",
            is_active: show_abilityless_active,
            on_toggle: toggle_abilityless,
        };
        let variants_button = CatalogVisibilityButtonProps {
            label: "All variants",
            title: "List every tier / upgrade variant separately",
            is_active: expand_variants_active,
            on_toggle: toggle_variants,
        };
        Self {
            abilityless_button,
            variants_button,
        }
    }
}
