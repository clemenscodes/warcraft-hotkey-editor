use super::fixtures;
use dioxus::prelude::*;
use hotkey_editor::services::editor_state::{
    DragFollower, DraggingSlot, DropTargetTile, EditorState,
};
use std::collections::{HashMap, HashSet};
use warcraft_api::UnitKind;
use warcraft_database::SearchField;
use warcraft_keybinds::{CustomKeys, GridSlotId};

/// App-specific story decorator that provides the three editor contexts the de-drilled
/// unit-detail components read rather than take as props: the loaded document, the
/// chosen grid layout, and the editor's UI-state bag. Every field is a fresh signal
/// seeded with the same default the app shell uses, so a presentational story renders
/// as it would in the live editor. Lives with the stories, mirroring `CustomKeysMount`
/// and `ToastMount`, and stays out of the domain-agnostic gallery framework.
#[component]
pub fn EditorMount(children: Element) -> Element {
    let loaded_keys = use_signal(|| None::<CustomKeys>);
    use_context_provider(|| loaded_keys);
    let grid_layout = use_signal(fixtures::sample_grid_layout);
    use_context_provider(|| grid_layout);
    let selected_slot = use_signal::<Option<GridSlotId>>(|| None);
    let selected_hero_level = use_signal::<u32>(|| 1);
    let selected_from_research = use_signal::<bool>(|| false);
    let selected_from_uprooted = use_signal::<bool>(|| false);
    let hotkey_assign_request = use_signal::<bool>(|| false);
    let tier_overrides = use_signal::<HashMap<String, usize>>(HashMap::new);
    let search_field = use_signal(SearchField::default);
    let collapsed_categories = use_signal::<HashSet<UnitKind>>(HashSet::new);
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
