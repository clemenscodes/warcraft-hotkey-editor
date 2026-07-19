use dioxus::prelude::*;
use std::time::Duration;
use warcraft_api::RaceSelection;
use warcraft_api::UnitCatalogGroup;
use warcraft_api::UnitModeSelection;

use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use crate::services::search_dialog::context::use_search_dialog_dismiss;
use crate::services::unit_catalog::context::use_unit_catalog;

pub(super) struct SearchDialogBodyPresentation {
    pub(super) search_value: ReadSignal<String>,
    pub(super) search_placeholder: &'static str,
    pub(super) on_input: EventHandler<FormEvent>,
    pub(super) on_keydown: EventHandler<KeyboardEvent>,
    pub(super) groups: Vec<UnitCatalogGroup>,
    pub(super) panel_open: bool,
    pub(super) active_filter_count: usize,
    pub(super) on_toggle_panel: EventHandler<MouseEvent>,
    pub(super) on_dismiss_panel: EventHandler<MouseEvent>,
}

pub(super) fn use_search_dialog_body() -> SearchDialogBodyPresentation {
    let navigation = use_view_navigation();
    let catalog = use_unit_catalog();
    let search_query = navigation.search_query();
    let mut raw_query = use_signal(|| search_query.read().clone());
    let mut debounce_generation: Signal<u32> = use_signal(|| 0);
    use_effect(move || {
        let committed = search_query.read().clone();
        if *raw_query.peek() != committed {
            raw_query.set(committed);
        }
    });
    let on_input = EventHandler::new(move |event: FormEvent| {
        let value = event.value();
        raw_query.set(value.clone());
        let current_generation: u32 = *debounce_generation.read();
        let next_generation = current_generation.wrapping_add(1);
        debounce_generation.set(next_generation);
        spawn(async move {
            let delay = Duration::from_millis(150);
            gloo_timers::future::sleep(delay).await;
            let generation_now: u32 = *debounce_generation.read();
            if generation_now == next_generation {
                navigation.set_search_query(value);
            }
        });
    });
    let editor = use_editor_state();
    let mut selected_slot = editor.selected_slot();
    let mut active_category = editor.active_category();
    let listing = catalog.listing();
    let first_result = listing.first_result().cloned();
    let groups = listing.into_groups();
    let dismiss = use_search_dialog_dismiss();
    let on_keydown = EventHandler::new(move |event: KeyboardEvent| {
        let key_string = event.key().to_string();
        match key_string.as_str() {
            "Escape" => {
                navigation.set_search_query(String::new());
            }
            "Enter" => {
                if let Some(entry) = first_result.as_ref() {
                    navigation.open_unit(entry.unit_id());
                    selected_slot.set(None);
                    active_category.set(entry.unit_kind());
                    if let Some(dismiss) = dismiss {
                        dismiss.dismiss();
                    }
                }
            }
            _ => {}
        }
    });
    let scope_signal = editor.search_race_scope();
    let unit_modes_signal = navigation.unit_modes();
    let show_abilityless_signal = editor.show_abilityless_units();
    let expand_variants_signal = editor.expand_variants();
    let scope = scope_signal.read().clone();
    let current_modes = *unit_modes_signal.read();
    let race_narrowed = scope != RaceSelection::All;
    let modes_changed = current_modes != UnitModeSelection::default();
    let show_plain_active = *show_abilityless_signal.read();
    let all_tiers_active = *expand_variants_signal.read();
    let active_flags = [
        race_narrowed,
        modes_changed,
        show_plain_active,
        all_tiers_active,
    ];
    let active_filter_count = active_flags.iter().filter(|flag| **flag).count();
    let mut panel_open_signal = use_signal(|| false);
    let panel_open = *panel_open_signal.read();
    let on_toggle_panel = EventHandler::new(move |_event: MouseEvent| {
        let next = !*panel_open_signal.peek();
        panel_open_signal.set(next);
    });
    let on_dismiss_panel = EventHandler::new(move |_event: MouseEvent| {
        panel_open_signal.set(false);
    });
    let search_value: ReadSignal<String> = raw_query.into();
    SearchDialogBodyPresentation {
        search_value,
        search_placeholder: "Search...",
        on_input,
        on_keydown,
        groups,
        panel_open,
        active_filter_count,
        on_toggle_panel,
        on_dismiss_panel,
    }
}
