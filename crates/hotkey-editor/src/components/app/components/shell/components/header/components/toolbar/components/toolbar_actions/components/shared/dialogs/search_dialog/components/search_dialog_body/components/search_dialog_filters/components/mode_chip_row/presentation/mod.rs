use dioxus::prelude::*;
use warcraft_api::UnitMode;

use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use crate::services::unit_catalog::context::use_unit_catalog;

/// One independent toggle in the filter row.
pub(super) struct FilterToggle {
    pub(super) key: &'static str,
    pub(super) label: &'static str,
    pub(super) title: &'static str,
    pub(super) is_active: bool,
    pub(super) on_pick: EventHandler<MouseEvent>,
}

/// The four toggles that narrow the catalog, side by side because they are the
/// same kind of control: each one is independently on or off and they merge into
/// one filter. Two come from navigation (the modes, which belong in the URL) and
/// two from editor state, but that split is plumbing — a player sees four
/// switches.
pub(super) fn use_mode_chip_row() -> Vec<FilterToggle> {
    let navigation = use_view_navigation();
    let editor = use_editor_state();
    let catalog = use_unit_catalog();
    let selected_slot = editor.selected_slot();
    let unit_modes = navigation.unit_modes();
    let current_modes = *unit_modes.read();
    let is_searching = catalog.filter().is_searching();
    let mut toggles: Vec<FilterToggle> = Vec::new();
    for mode in [UnitMode::Melee, UnitMode::Campaign] {
        let is_active = !is_searching && current_modes.includes(mode);
        let on_pick = EventHandler::new(move |_event: MouseEvent| {
            navigation.toggle_mode(mode, selected_slot);
        });
        let key = match mode {
            UnitMode::Melee => "melee",
            UnitMode::Campaign => "campaign",
        };
        let label = match mode {
            UnitMode::Melee => "Melee",
            UnitMode::Campaign => "Campaign",
        };
        let title = match mode {
            UnitMode::Melee => "List the units available in melee",
            UnitMode::Campaign => "List the units available in campaign",
        };
        let toggle = FilterToggle {
            key,
            label,
            title,
            is_active,
            on_pick,
        };
        toggles.push(toggle);
    }
    let mut show_abilityless = editor.show_abilityless_units();
    let abilityless_active = *show_abilityless.read();
    let on_abilityless = EventHandler::new(move |_event: MouseEvent| {
        let next = !*show_abilityless.peek();
        show_abilityless.set(next);
    });
    // "No abilities" said the opposite of what it does. On, this ADDS the units
    // whose only commands are the standard ones — move, attack, stop — which have
    // nothing bindable of their own and are hidden by default.
    let abilityless = FilterToggle {
        key: "abilityless",
        label: "Plain units",
        title: "Also list units with no abilities of their own, only the standard commands",
        is_active: abilityless_active,
        on_pick: on_abilityless,
    };
    toggles.push(abilityless);
    let mut expand_variants = editor.expand_variants();
    let variants_active = *expand_variants.read();
    let on_variants = EventHandler::new(move |_event: MouseEvent| {
        let next = !*expand_variants.peek();
        expand_variants.set(next);
    });
    // Off, a unit that exists at several tiers shows only its strongest, and an
    // edit there merges down onto the weaker tiers. On, every tier is its own row.
    let variants = FilterToggle {
        key: "variants",
        label: "All tiers",
        title: "List every tier separately instead of only the strongest, which edits merge down from",
        is_active: variants_active,
        on_pick: on_variants,
    };
    toggles.push(variants);
    toggles
}
