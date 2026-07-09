use super::logic::CatalogListingInputs;
use super::props::{SearchKeydownInputs, UnitListInputs, UnitListModel, UnitListProps};
use super::state::UnitListState;
use crate::services::focus::context::use_focus_coordinator;
use crate::services::focus::coordinator::FocusTarget;
use dioxus::prelude::*;
use std::time::Duration;
use warcraft_database::{CatalogVisibility, SearchField};

/// The debounced search box's shaped state: the immediate `raw_query` the input
/// shows, plus its input and clear handlers. Owns the `raw_query` and generation-
/// counter signals and the effect that resyncs `raw_query` when the committed
/// query changes underneath it. The input handler commits the query 150ms after
/// the last keystroke, guarded by the generation counter so only the final
/// keystroke wins; the clear handler resets both queries and bumps the counter.
pub(super) struct DebouncedSearch {
    pub(super) raw_query: Signal<String>,
    pub(super) on_input: EventHandler<FormEvent>,
    pub(super) on_clear: EventHandler<()>,
}

fn use_debounced_search(search_query: Signal<String>) -> DebouncedSearch {
    let mut search_query = search_query;
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
                search_query.set(value);
            }
        });
    });
    let on_clear = EventHandler::new(move |_: ()| {
        raw_query.set(String::new());
        search_query.set(String::new());
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

/// The search box's keydown handler: Escape either clears a non-empty query or
/// hands focus to the unit card, and Enter selects the first result and hands
/// focus to the unit card. Owns none of its inputs; the composed hook wires them.
fn use_search_keydown(inputs: SearchKeydownInputs) -> EventHandler<KeyboardEvent> {
    let SearchKeydownInputs {
        raw_query,
        on_clear,
        focus,
        first_result_id,
        first_result_kind,
        mut selected_unit_id,
        mut selected_slot,
        mut active_category,
    } = inputs;
    EventHandler::new(move |event: KeyboardEvent| {
        let key_string = event.key().to_string();
        match key_string.as_str() {
            "Escape" => {
                let current_raw = raw_query.read().clone();
                if current_raw.is_empty() {
                    focus.request(FocusTarget::UnitCard);
                } else {
                    on_clear.call(());
                }
            }
            "Enter" => {
                if let (Some(unit_id), Some(unit_kind)) = (first_result_id, first_result_kind) {
                    selected_unit_id.set(Some(unit_id));
                    selected_slot.set(None);
                    active_category.set(unit_kind);
                    focus.request(FocusTarget::UnitCard);
                }
            }
            _ => {}
        }
    })
}

/// Reads the list's signals, runs the debounced search, computes the derived
/// catalog state, and shapes every child's props so the body stays pure RSX.
///
/// The catalog walk itself (`UnitListing::resolve`) is memoized on the race,
/// mode, committed query, search field, and visibility — the values it
/// actually depends on — so it does not re-run on unrelated re-renders such as
/// a unit selection.
pub(super) fn use_unit_list(props: &UnitListProps) -> UnitListModel {
    let active_race = props.active_race;
    let unit_mode = props.unit_mode;
    let selected_unit_id = props.selected_unit_id;
    let selected_slot = props.selected_slot;
    let search_query = props.search_query;
    let search_field = props.search_field;
    let show_abilityless_units = props.show_abilityless_units;
    let expand_variants = props.expand_variants;
    let collapsed_categories = props.collapsed_categories;
    let race = *active_race.read();
    let mode = *unit_mode.read();
    let committed_query = search_query.read().clone();
    let current_search_field = *search_field.read();
    let show_abilityless_active = *show_abilityless_units.read();
    let expand_variants_active = *expand_variants.read();
    let visibility = CatalogVisibility::new(show_abilityless_active, expand_variants_active);
    let search_placeholder = match current_search_field {
        SearchField::UnitName => "Search units…",
        SearchField::Ability => "Search by ability…",
    };
    let search = use_debounced_search(search_query);
    let listing_memo = use_memo(move || {
        let listing_race = *active_race.read();
        let listing_mode = *unit_mode.read();
        let listing_query = search_query.read().clone();
        let listing_search_field = *search_field.read();
        let listing_show_abilityless = *show_abilityless_units.read();
        let listing_expand_variants = *expand_variants.read();
        let listing_visibility =
            CatalogVisibility::new(listing_show_abilityless, listing_expand_variants);
        let inputs = CatalogListingInputs {
            race: listing_race,
            mode: listing_mode,
            query: listing_query,
            search_field: listing_search_field,
            visibility: listing_visibility,
        };
        inputs.into_listing()
    });
    let listing = listing_memo();
    let state = UnitListState::new(
        committed_query,
        selected_unit_id,
        collapsed_categories,
        listing,
    );
    let active_category = state.active_category();
    let active_kind = state.active_kind();
    let search_active = state.search_active();
    let first_result_id = state.first_result_id();
    let first_result_kind = state.first_result_kind();
    let raw_query = search.raw_query;
    let on_clear = search.on_clear;
    let on_input = search.on_input;
    let focus = use_focus_coordinator();
    let keydown_inputs = SearchKeydownInputs {
        raw_query,
        on_clear,
        focus,
        first_result_id,
        first_result_kind,
        selected_unit_id,
        selected_slot,
        active_category,
    };
    let on_keydown = use_search_keydown(keydown_inputs);
    let inputs = UnitListInputs {
        state,
        race,
        mode,
        current_search_field,
        visibility,
        selected_unit_id,
        selected_slot,
        collapsed_categories,
        search_field,
        show_abilityless_units,
        expand_variants,
        search_active,
        active_kind,
        active_category,
        raw_query,
        search_placeholder,
        on_input,
        on_keydown,
    };
    UnitListModel::from(inputs)
}
