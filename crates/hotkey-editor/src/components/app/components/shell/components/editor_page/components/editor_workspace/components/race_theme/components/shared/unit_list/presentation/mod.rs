use super::model::SearchKeydownInputs;
use super::model::UnitListInputs;
use super::model::UnitListModel;
use super::state::UnitListState;
use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use crate::services::unit_catalog::context::use_unit_catalog;
use dioxus::prelude::*;
use std::time::Duration;
use warcraft_api::SearchField;

pub(super) struct DebouncedSearch {
    pub(super) raw_query: Signal<String>,
    pub(super) on_input: EventHandler<FormEvent>,
    pub(super) on_clear: EventHandler<()>,
}

fn use_debounced_search(navigation: ViewNavigationContext) -> DebouncedSearch {
    let search_query = navigation.search_query();
    let mut raw_query = use_signal(|| search_query.read().clone());
    let mut debounce_gen: Signal<u32> = use_signal(|| 0);
    use_effect(move || {
        let committed = search_query.read().clone();
        if *raw_query.peek() != committed {
            raw_query.set(committed);
        }
    });
    let on_input = EventHandler::new(move |event: FormEvent| {
        let value = event.value();
        raw_query.set(value.clone());
        let current_gen: u32 = *debounce_gen.read();
        let next_gen = current_gen.wrapping_add(1);
        debounce_gen.set(next_gen);
        spawn(async move {
            let delay = Duration::from_millis(150);
            gloo_timers::future::sleep(delay).await;
            let gen_now: u32 = *debounce_gen.read();
            if gen_now == next_gen {
                navigation.set_search_query(value);
            }
        });
    });
    let on_clear = EventHandler::new(move |_: ()| {
        raw_query.set(String::new());
        let cleared_query = String::new();
        navigation.set_search_query(cleared_query);
        let current_gen: u32 = *debounce_gen.read();
        let next_gen = current_gen.wrapping_add(1);
        debounce_gen.set(next_gen);
    });
    DebouncedSearch {
        raw_query,
        on_input,
        on_clear,
    }
}

fn use_search_keydown(inputs: SearchKeydownInputs) -> EventHandler<KeyboardEvent> {
    let SearchKeydownInputs {
        raw_query,
        on_clear,
        first_result,
        navigation,
        mut selected_slot,
        mut active_category,
    } = inputs;
    EventHandler::new(move |event: KeyboardEvent| {
        let key_string = event.key().to_string();
        match key_string.as_str() {
            "Escape" => {
                let current_raw = raw_query.read().clone();
                if !current_raw.is_empty() {
                    on_clear.call(());
                }
            }
            "Enter" => {
                if let Some(first_result) = first_result {
                    // Searching drops the race filter, so a hit can belong to any
                    // race. `open_unit` takes the race and the mode from the unit
                    // itself, which is what keeps the theme with the unit instead
                    // of stranding a Demon Hunter under the Human theme.
                    navigation.open_unit(first_result.id());
                    selected_slot.set(None);
                    active_category.set(first_result.kind());
                }
            }
            _ => {}
        }
    })
}

pub(super) fn use_unit_list() -> UnitListModel {
    let navigation = use_view_navigation();
    let editor = use_editor_state();
    let selected_slot = editor.selected_slot();
    let search_field = editor.search_field();
    let active_category = editor.active_category();
    let current_search_field = *search_field.read();
    let search_placeholder = match current_search_field {
        SearchField::UnitName => "Search units…",
        SearchField::Ability => "Search by ability…",
    };
    let search = use_debounced_search(navigation);
    let catalog = use_unit_catalog();
    let listing = catalog.listing();
    let state = UnitListState::from(listing);
    let first_result = state.first_result();
    let raw_query = search.raw_query;
    let on_clear = search.on_clear;
    let on_input = search.on_input;
    let keydown_inputs = SearchKeydownInputs {
        raw_query,
        on_clear,
        first_result,
        navigation,
        selected_slot,
        active_category,
    };
    let on_keydown = use_search_keydown(keydown_inputs);
    let inputs = UnitListInputs {
        state,
        raw_query,
        search_placeholder,
        on_input,
        on_keydown,
    };
    UnitListModel::from(inputs)
}
