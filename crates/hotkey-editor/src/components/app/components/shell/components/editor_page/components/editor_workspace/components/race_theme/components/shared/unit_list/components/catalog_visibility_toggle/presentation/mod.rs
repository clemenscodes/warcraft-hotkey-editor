use dioxus::prelude::*;

pub(super) struct CatalogVisibilitySignals {
    pub(super) show_abilityless_units: Signal<bool>,
    pub(super) expand_variants: Signal<bool>,
}

pub(super) struct CatalogVisibilityToggleModel {
    pub(super) abilityless_is_active: bool,
    pub(super) variants_is_active: bool,
    pub(super) toggle_abilityless: EventHandler<MouseEvent>,
    pub(super) toggle_variants: EventHandler<MouseEvent>,
}

impl From<CatalogVisibilitySignals> for CatalogVisibilityToggleModel {
    fn from(signals: CatalogVisibilitySignals) -> Self {
        let mut show_abilityless_units = signals.show_abilityless_units;
        let mut expand_variants = signals.expand_variants;
        let abilityless_is_active = *show_abilityless_units.read();
        let variants_is_active = *expand_variants.read();
        let toggle_abilityless = EventHandler::new(move |_event: MouseEvent| {
            show_abilityless_units.set(!abilityless_is_active);
        });
        let toggle_variants = EventHandler::new(move |_event: MouseEvent| {
            expand_variants.set(!variants_is_active);
        });
        Self {
            abilityless_is_active,
            variants_is_active,
            toggle_abilityless,
            toggle_variants,
        }
    }
}
