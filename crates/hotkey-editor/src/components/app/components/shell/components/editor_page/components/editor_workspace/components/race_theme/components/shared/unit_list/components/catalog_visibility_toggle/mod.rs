pub mod components;
mod data;
mod presentation;
mod style;

use crate::services::editor_state::context::use_editor_state;
use components::catalog_visibility_switch::CatalogVisibilitySwitch;
use dioxus::prelude::*;
use presentation::{CatalogVisibilitySignals, CatalogVisibilityToggleModel};
use style::CLASS;
use super::shared::filter_group_label::FilterGroupLabel;
use tw_macro::assert_component;

#[component]
pub fn CatalogVisibilityToggle() -> Element {
    let editor = use_editor_state();
    let signals = CatalogVisibilitySignals {
        show_abilityless_units: editor.show_abilityless_units(),
        expand_variants: editor.expand_variants(),
    };
    let CatalogVisibilityToggleModel {
        abilityless_is_active,
        variants_is_active,
        toggle_abilityless,
        toggle_variants,
    } = CatalogVisibilityToggleModel::from(signals);
    rsx! {
        div {
            class: CLASS,
            role: "group",
            aria_label: "Show",
            FilterGroupLabel { text: data::SHOW_LABEL }
            CatalogVisibilitySwitch {
                label: "No abilities",
                title: "Show units without abilities (for stats)",
                is_active: abilityless_is_active,
                on_toggle: toggle_abilityless,
            }
            CatalogVisibilitySwitch {
                label: "All variants",
                title: "List every tier / upgrade variant separately",
                is_active: variants_is_active,
                on_toggle: toggle_variants,
            }
        }
    }
}

assert_component!(CatalogVisibilityToggle);
