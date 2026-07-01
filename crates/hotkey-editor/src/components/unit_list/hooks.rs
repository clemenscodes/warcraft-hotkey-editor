use super::components::catalog_visibility_toggle::CatalogVisibilityToggleProps;
use super::components::search_field_toggle::SearchFieldToggleProps;
use super::components::unit_list_search::UnitListSearchProps;
use super::mobile_category_tab::MobileCategoryTabProps;
use super::props::UnitListProps;
use super::state::UnitListState;
use super::unit_category_section::UnitCategorySectionProps;
use super::unit_kind_data_attr;
use crate::services::focus::modality::FocusModality;
use dioxus::document;
use dioxus::prelude::*;
use std::time::Duration;
use warcraft_api::UnitKind;
use warcraft_database::{CatalogVisibility, SearchField, UnitKindHelpers};

const MOBILE_CATEGORY_ORDER: [UnitKind; 4] = [
    UnitKind::Hero,
    UnitKind::Soldier,
    UnitKind::Worker,
    UnitKind::Building,
];

/// The unit list's shaped view: the data attributes for the panel, and the
/// finished props for every child (the two toggles, the search box, the mobile
/// category tabs, and the category sections).
pub(super) struct UnitListModel {
    pub(super) active_category_attr: &'static str,
    pub(super) search_active: bool,
    pub(super) search_field_toggle: SearchFieldToggleProps,
    pub(super) catalog_visibility_toggle: CatalogVisibilityToggleProps,
    pub(super) search: UnitListSearchProps,
    pub(super) mobile_tabs: Vec<MobileCategoryTabProps>,
    pub(super) sections: Vec<UnitCategorySectionProps>,
}

/// Reads the list's signals, runs the debounced search, computes the derived
/// catalog state, and shapes every child's props so the body stays pure RSX.
pub(super) fn use_unit_list(props: &UnitListProps) -> UnitListModel {
    let active_race = props.active_race;
    let unit_mode = props.unit_mode;
    let mut selected_unit_id = props.selected_unit_id;
    let mut selected_slot = props.selected_slot;
    let mut search_query = props.search_query;
    let search_field = props.search_field;
    let show_abilityless_units = props.show_abilityless_units;
    let expand_variants = props.expand_variants;
    let collapsed_categories = props.collapsed_categories;
    let current_search_field = *search_field.read();
    let show_abilityless_active = *show_abilityless_units.read();
    let expand_variants_active = *expand_variants.read();
    let visibility = CatalogVisibility::new(show_abilityless_active, expand_variants_active);
    let search_placeholder = match current_search_field {
        SearchField::UnitName => "Search units…",
        SearchField::Ability => "Search by ability…",
    };
    let mut raw_query = use_signal(|| search_query.read().clone());
    let mut debounce_gen: Signal<u32> = use_signal(|| 0);
    use_effect(move || {
        let committed = search_query.read().clone();
        if *raw_query.peek() != committed {
            raw_query.set(committed);
        }
    });
    let state = UnitListState::new(
        active_race,
        unit_mode,
        search_query,
        current_search_field,
        selected_unit_id,
        collapsed_categories,
        visibility,
    );
    let mut active_category_signal = state.active_category();
    let active_kind = state.active_kind();
    let search_active = state.search_active();
    let race = state.race();
    let mode = state.mode();
    let first_result_id = state.first_result_id().map(str::to_owned);
    let first_result_kind = state.first_result_kind();
    let on_keydown = EventHandler::new(move |event: KeyboardEvent| {
        let key_string = event.key().to_string();
        match key_string.as_str() {
            "Escape" => {
                let current_raw = raw_query.read().clone();
                if current_raw.is_empty() {
                    let focus_script = "document.body.setAttribute('data-kb-modality', ''); const card = document.querySelector('.unit-card'); if (card) card.focus();";
                    document::eval(focus_script);
                } else {
                    raw_query.set(String::new());
                    search_query.set(String::new());
                    let current_gen: u32 = *debounce_gen.read();
                    let next_gen = current_gen.wrapping_add(1);
                    debounce_gen.set(next_gen);
                }
            }
            "Enter" => {
                if let (Some(unit_id), Some(unit_kind)) =
                    (first_result_id.clone(), first_result_kind)
                {
                    selected_unit_id.set(Some(unit_id));
                    selected_slot.set(None);
                    active_category_signal.set(unit_kind);
                    FocusModality::after_render(".unit-card.selected, .unit-card");
                }
            }
            _ => {}
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
    let active_category_attr = unit_kind_data_attr(active_kind);
    let search_field_toggle = SearchFieldToggleProps { search_field };
    let catalog_visibility_toggle = CatalogVisibilityToggleProps {
        show_abilityless_units,
        expand_variants,
    };
    let search = UnitListSearchProps {
        value: raw_query.into(),
        placeholder: search_placeholder,
        on_input,
        on_keydown,
    };
    let mobile_tabs = MOBILE_CATEGORY_ORDER
        .iter()
        .map(|&kind| {
            let is_active = kind == active_kind;
            MobileCategoryTabProps {
                kind,
                is_active,
                active_category: active_category_signal,
            }
        })
        .collect();
    let sections = state
        .category_kinds()
        .iter()
        .map(|&kind| {
            let category_label = UnitKindHelpers::category_label(kind).to_owned();
            let is_collapsed = state.collapsed_snapshot().contains(&kind);
            let query = state.query_snapshot().to_owned();
            let active_unit_id = state.active_unit_id().map(str::to_owned);
            UnitCategorySectionProps {
                category_kind: kind,
                category_label,
                is_collapsed,
                collapsed_categories,
                race,
                mode,
                query,
                search_field: current_search_field,
                visibility,
                active_unit_id,
                selected_unit_id,
                selected_slot,
                active_category: active_category_signal,
            }
        })
        .collect();
    UnitListModel {
        active_category_attr,
        search_active,
        search_field_toggle,
        catalog_visibility_toggle,
        search,
        mobile_tabs,
        sections,
    }
}
