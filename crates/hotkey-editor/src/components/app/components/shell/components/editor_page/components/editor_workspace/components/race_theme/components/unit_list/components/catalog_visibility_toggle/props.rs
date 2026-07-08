use dioxus::prelude::*;

/// The visibility toggle owns the two boolean signals it reads and flips.
#[derive(Props, Clone, PartialEq)]
pub struct CatalogVisibilityToggleProps {
    pub show_abilityless_units: Signal<bool>,
    pub expand_variants: Signal<bool>,
}
