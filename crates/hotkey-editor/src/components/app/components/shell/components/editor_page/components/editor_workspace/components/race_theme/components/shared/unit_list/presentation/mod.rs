use warcraft_api::CatalogVisibility;
use warcraft_api::Race;
use warcraft_api::SearchField;
use warcraft_api::UnitListing;
use warcraft_api::UnitListingRequest;
use warcraft_api::UnitMode;

/// The inputs the memoized catalog walk reads — race, mode, committed query,
/// search field, and catalog visibility. It orchestrates the domain call
/// [`UnitListing::resolve`]; the walk itself lives in `warcraft-keybinds`.
#[derive(Clone, PartialEq, Debug)]
pub(super) struct CatalogListingInputs {
    pub(super) race: Race,
    pub(super) mode: UnitMode,
    pub(super) query: String,
    pub(super) search_field: SearchField,
    pub(super) visibility: CatalogVisibility,
}

impl CatalogListingInputs {
    /// Consume these inputs into the domain [`UnitListing`]. A consuming `into_*`
    /// conversion rather than `From`/`Into`, since the output is the foreign domain
    /// type (the orphan rule forbids a `From` impl in the renderer crate).
    pub(super) fn into_listing(self) -> UnitListing {
        let Self {
            race,
            mode,
            query,
            search_field,
            visibility,
        } = self;
        let request = UnitListingRequest::new(race, mode, query, search_field, visibility);
        UnitListing::resolve(&request)
    }
}
use super::model::SearchKeydownInputs;
use super::model::UnitListInputs;
use super::model::UnitListModel;
use super::state::UnitListState;
use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;
use std::time::Duration;

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

/// The search box's keydown handler: Escape clears a non-empty query, and Enter
/// selects the first result. Owns none of its inputs; the composed hook wires them.
fn use_search_keydown(inputs: SearchKeydownInputs) -> EventHandler<KeyboardEvent> {
    let SearchKeydownInputs {
        raw_query,
        on_clear,
        first_result,
        mut selected_unit_id,
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
                    selected_unit_id.set(Some(first_result.id()));
                    selected_slot.set(None);
                    active_category.set(first_result.kind());
                }
            }
            _ => {}
        }
    })
}

/// Reads the list's signals from context, runs the debounced search, computes the
/// derived catalog state, and shapes every child's props so the body stays pure RSX.
///
/// The catalog walk itself (`UnitListing::resolve`) is memoized on the race, mode,
/// committed query, search field, and visibility — the values it actually depends on —
/// so it does not re-run on unrelated re-renders such as a unit selection.
pub(super) fn use_unit_list() -> UnitListModel {
    let navigation = use_view_navigation();
    let editor = use_editor_state();
    let active_race = navigation.active_race();
    let unit_mode = navigation.unit_mode();
    let selected_unit_id = navigation.selected_unit_id();
    let search_query = navigation.search_query();
    let selected_slot = editor.selected_slot();
    let search_field = editor.search_field();
    let show_abilityless_units = editor.show_abilityless_units();
    let expand_variants = editor.expand_variants();
    let active_category = editor.active_category();
    let current_search_field = *search_field.read();
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
        let listing_visibility = CatalogVisibility {
            include_abilityless: listing_show_abilityless,
            expand_variants: listing_expand_variants,
        };
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
    let state = UnitListState::new(listing);
    let first_result = state.first_result();
    let raw_query = search.raw_query;
    let on_clear = search.on_clear;
    let on_input = search.on_input;
    let keydown_inputs = SearchKeydownInputs {
        raw_query,
        on_clear,
        first_result,
        selected_unit_id,
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
