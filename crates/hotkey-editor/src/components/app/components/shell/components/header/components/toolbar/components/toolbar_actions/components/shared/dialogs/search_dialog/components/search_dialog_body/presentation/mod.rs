use dioxus::prelude::*;
use std::time::Duration;
use warcraft_api::SearchField;
use warcraft_api::UnitCatalogGroup;

use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use crate::services::unit_catalog::context::use_unit_catalog;

/// The search dialog's content, already shaped.
///
/// This dialog is the whole reason a phone can navigate at all: the editor page
/// mounts the pager instead of the aside below 768px, and nothing in that tree
/// calls `select_race` or `select_mode`. Without it a phone reaches another race
/// only by swiping through every unit in the game, in one flat list, until it
/// crosses the boundary.
///
/// Picking a result is not handled here — the unit card already opens the unit,
/// clears the slot and sets the category. Duplicating that would be a second
/// place to get it wrong.
pub(super) struct SearchDialogBodyPresentation {
    pub(super) search_value: ReadSignal<String>,
    pub(super) search_placeholder: &'static str,
    pub(super) on_input: EventHandler<FormEvent>,
    pub(super) on_keydown: EventHandler<KeyboardEvent>,
    pub(super) groups: Vec<UnitCatalogGroup>,
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
    // The query lives in the URL, so every keystroke would otherwise be its own
    // history entry and its own catalog pass. The aside debounces the same way.
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
    let search_field = editor.search_field();
    let mut selected_slot = editor.selected_slot();
    let mut active_category = editor.active_category();
    let current_search_field = *search_field.read();
    let search_placeholder = match current_search_field {
        SearchField::UnitName => "Search every race…",
        SearchField::Ability => "Search by ability…",
    };
    let listing = catalog.listing();
    let first_result = listing.first_result().cloned();
    let groups = listing.into_groups();
    let on_keydown = EventHandler::new(move |event: KeyboardEvent| {
        let key_string = event.key().to_string();
        match key_string.as_str() {
            "Escape" => {
                navigation.set_search_query(String::new());
            }
            "Enter" => {
                // The same triple the unit card does: a search spans every race,
                // so `open_unit` takes the race and the mode from the hit itself.
                if let Some(entry) = first_result.as_ref() {
                    navigation.open_unit(entry.unit_id());
                    selected_slot.set(None);
                    active_category.set(entry.unit_kind());
                }
            }
            _ => {}
        }
    });
    let search_value: ReadSignal<String> = raw_query.into();
    SearchDialogBodyPresentation {
        search_value,
        search_placeholder,
        on_input,
        on_keydown,
        groups,
    }
}
