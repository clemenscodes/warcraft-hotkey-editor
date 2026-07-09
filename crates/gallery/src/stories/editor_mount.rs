use super::fixtures;
use dioxus::prelude::*;
use hotkey_editor::services::editor_state::{
    DragFollower, DraggingSlot, DropTargetTile, EditorState,
};
use hotkey_editor::services::navigation::app_view::AppView;
use hotkey_editor::services::navigation::view_navigation::ViewNavigationContext;
use std::collections::{HashMap, HashSet};
use warcraft_api::SearchField;
use warcraft_api::UnitKind;
use warcraft_api::WarcraftObjectId;
use warcraft_api::{Race, UnitMode};
use warcraft_keybinds::{CustomKeys, GridSlotId};

/// App-specific story decorator that provides every editor context the de-drilled unit
/// list and detail components read rather than take as props: the navigation state (the
/// active race and selected unit a story wants to show), the loaded document, the chosen
/// grid layout, and the editor's UI-state bag. Every field is a fresh signal seeded with
/// the same default the app shell uses (or the story's chosen race/unit), so a
/// presentational story renders as it would in the live editor. Lives with the stories,
/// mirroring `CustomKeysMount` and `ToastMount`, and stays out of the domain-agnostic
/// gallery framework.
#[component]
pub fn EditorMount(
    #[props(default = Race::Human)] active_race: Race,
    #[props(default)] selected_unit_id: Option<WarcraftObjectId>,
    children: Element,
) -> Element {
    let current_view = use_signal(|| AppView::Editor);
    let active_race_signal = use_signal(|| active_race);
    let unit_mode = use_signal(|| UnitMode::Melee);
    let selected_unit_id_signal = use_signal(|| selected_unit_id);
    let search_query = use_signal(String::new);
    let navigation = ViewNavigationContext::new(
        current_view,
        active_race_signal,
        unit_mode,
        selected_unit_id_signal,
        search_query,
    );
    use_context_provider(|| navigation);
    let loaded_keys = use_signal(|| None::<CustomKeys>);
    use_context_provider(|| loaded_keys);
    let grid_layout = use_signal(fixtures::sample_grid_layout);
    use_context_provider(|| grid_layout);
    let selected_slot = use_signal::<Option<GridSlotId>>(|| None);
    let selected_hero_level = use_signal::<u32>(|| 1);
    let selected_from_research = use_signal::<bool>(|| false);
    let selected_from_uprooted = use_signal::<bool>(|| false);
    let hotkey_assign_request = use_signal::<bool>(|| false);
    let tier_overrides = use_signal::<HashMap<WarcraftObjectId, usize>>(HashMap::new);
    let search_field = use_signal(SearchField::default);
    let collapsed_categories = use_signal::<HashSet<UnitKind>>(HashSet::new);
    let active_category = use_signal::<UnitKind>(|| UnitKind::Soldier);
    let show_abilityless_units = use_signal::<bool>(|| false);
    let expand_variants = use_signal::<bool>(|| false);
    let dragging_slot = use_signal::<Option<DraggingSlot>>(|| None);
    let drop_target_tile = use_signal::<Option<DropTargetTile>>(|| None);
    let drag_follower = use_signal::<Option<DragFollower>>(|| None);
    let update_hotkeys_on_move = use_signal::<bool>(|| true);
    let editor_state = EditorState::new(
        selected_slot,
        selected_hero_level,
        selected_from_research,
        selected_from_uprooted,
        hotkey_assign_request,
        tier_overrides,
        search_field,
        collapsed_categories,
        active_category,
        show_abilityless_units,
        expand_variants,
        dragging_slot,
        drop_target_tile,
        drag_follower,
        update_hotkeys_on_move,
    );
    use_context_provider(|| editor_state);
    rsx! {
        {children}
    }
}
