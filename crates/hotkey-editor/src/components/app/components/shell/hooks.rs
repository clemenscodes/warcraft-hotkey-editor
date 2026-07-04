use crate::components::app::components::shell::route_sync::NavDecision;
use crate::components::app::components::shell::style;
use crate::components::app::route::Route;
use crate::services::collision_selection::CollisionSelection;
use crate::services::customkeys::persistence::{CustomKeysPersistence, OnboardingPersistence};
use crate::services::customkeys::service::CustomKeysService;
use crate::services::customkeys::upload_status::UploadStatus;
use crate::services::editor_state::EditorState;
use crate::services::editor_state::{DragFollower, DraggingSlot, DropTargetTile};
use crate::services::focus::navigation::{FocusNavigation, FocusedElementInfo};
use crate::services::navigation::app_view::{AppView, CollisionKind};
use crate::services::navigation::editor_nav::DecodedEditorNav;
use crate::services::navigation::nav_snapshot::NavSnapshot;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use crate::services::overlay_state::OverlayState;
use crate::services::resolve_selection::ResolveSelection;
use crate::services::undo::{EditorSnapshot, UndoHistory};
use crate::styling::ClassList;
use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::GridLayout;
use warcraft_keybinds::GridSlotId;

/// The two things the [`Shell`](super::Shell) body needs: the root class and the
/// app-level key handler. Every piece of app-wide state the shell owns is handed to
/// the page tree, the header, and the header's dialog hosts through context, so the
/// body is a flat list of children — no god-bag of props.
pub(super) struct ShellModel {
    pub(super) class: ClassList,
    pub(super) handle_keydown: EventHandler<KeyboardEvent>,
}

/// Build the app shell's full model: load the document and grid layout from storage,
/// wire the persistence/undo effects, own every app-wide signal, provide the
/// contexts the header and the routed pages read, and run the URL-sync push effect.
///
/// The pages reconcile the live route back into these signals (the read side of the
/// URL contract), so this hook only pushes: on any state change it builds the route
/// the signals now describe and diffs it against the address bar, pushing a new
/// history entry for a page/selection change, replacing for an entry pick, and
/// running the search-typing session for a query edit. Comparing against the live
/// route (read without subscribing) is what keeps a browser back/forward from being
/// echoed straight back.
pub(super) fn use_shell() -> ShellModel {
    let loaded_keys = use_signal::<Option<CustomKeys>>(|| {
        let stored_text = CustomKeysPersistence::load_text();
        let initial_file = match stored_text {
            Some(stored) => CustomKeys::from_text(stored.as_str()),
            None => CustomKeys::from_text(""),
        };
        let canonical_text = initial_file.to_string();
        CustomKeysPersistence::save_text(&canonical_text);
        Some(initial_file)
    });
    use_effect(move || {
        let read_guard = loaded_keys.read();
        let Some(file) = read_guard.as_ref() else {
            return;
        };
        let canonical_text = file.normalize().to_string();
        CustomKeysPersistence::save_text(&canonical_text);
    });
    let custom_keys_service = CustomKeysService::new(loaded_keys);
    use_context_provider(|| custom_keys_service);
    let grid_layout = use_signal::<GridLayout>(|| {
        CustomKeysPersistence::load_grid_layout().unwrap_or_else(GridLayout::qwerty_grid)
    });
    use_effect(move || {
        let snapshot = *grid_layout.read();
        CustomKeysPersistence::save_grid_layout(snapshot);
    });
    let update_hotkeys_on_move =
        use_signal::<bool>(CustomKeysPersistence::load_update_hotkeys_on_move);
    use_effect(move || {
        let enabled = *update_hotkeys_on_move.read();
        CustomKeysPersistence::save_update_hotkeys_on_move(enabled);
    });
    let undo_history = UndoHistory::use_history(loaded_keys, grid_layout);
    use_context_provider(|| undo_history);
    use_effect(move || {
        let keys_text = loaded_keys
            .read()
            .as_ref()
            .map(|file| file.normalize().to_string())
            .unwrap_or_default();
        let grid_layout_text = grid_layout.read().to_storage_string();
        let snapshot = EditorSnapshot::new(keys_text, grid_layout_text);
        undo_history.record(snapshot);
    });
    use_hook(move || undo_history.install_keyboard_shortcuts());
    use_effect(move || undo_history.handle_keyboard_request());

    let initial_route = router().current::<Route>();
    let initial_snapshot = NavSnapshot::from(&initial_route);
    // A bare or partial URL (`/`, `/collisions`) decodes to the same state as its
    // fully-materialized form (`/?race=human&mode=melee&unit=…`). Canonicalizing it on
    // entry keeps the address bar a complete, shareable description of what is shown —
    // the behavior the app has always had. It is a distinct concern from the
    // signal→URL sync below (which never fires here, since the decoded state already
    // matches), so it gets its own one-time replace.
    let initial_canonical_route = Route::from(&initial_snapshot);
    let needs_canonicalize = initial_route != initial_canonical_route;
    let initial_nav = match &initial_snapshot {
        NavSnapshot::Editor(nav) => nav.clone(),
        _ => DecodedEditorNav::decode(None, None, None, None),
    };
    let initial_view = match &initial_snapshot {
        NavSnapshot::Editor(_) => AppView::Editor,
        NavSnapshot::Collisions { kind, .. } => AppView::Collisions { kind: *kind },
        NavSnapshot::Resolve { .. } => AppView::Resolve,
    };
    let initial_race = initial_nav.race;
    let initial_mode = initial_nav.unit_mode;
    let initial_unit_id = initial_nav.selected_unit_id.clone();
    let initial_search = initial_nav.search_query.clone();
    let initial_island = match &initial_snapshot {
        NavSnapshot::Collisions {
            kind: CollisionKind::Positions,
            entry,
        } => entry.clone(),
        _ => None,
    };
    let initial_hotkey_unit = match &initial_snapshot {
        NavSnapshot::Collisions {
            kind: CollisionKind::Hotkeys,
            entry,
        } => entry.clone(),
        _ => None,
    };
    let initial_unit_position = match &initial_snapshot {
        NavSnapshot::Collisions {
            kind: CollisionKind::UnitPositions,
            entry,
        } => entry.clone(),
        _ => None,
    };
    let initial_move_category = match &initial_snapshot {
        NavSnapshot::Resolve { entry } => entry.clone(),
        _ => None,
    };

    let current_view = use_signal::<AppView>(move || initial_view);
    let active_race = use_signal(move || initial_race);
    let unit_mode = use_signal(move || initial_mode);
    let selected_unit_id = use_signal::<Option<String>>(move || initial_unit_id);
    let selected_slot = use_signal::<Option<GridSlotId>>(|| None);
    let selected_from_research = use_signal::<bool>(|| false);
    let selected_from_uprooted = use_signal::<bool>(|| false);
    let hotkey_assign_request = use_signal::<bool>(|| false);
    let tier_overrides = use_signal::<HashMap<String, usize>>(HashMap::new);
    let mut dragging_slot = use_signal::<Option<DraggingSlot>>(|| None);
    let mut drop_target_tile = use_signal::<Option<DropTargetTile>>(|| None);
    let mut drag_follower = use_signal::<Option<DragFollower>>(|| None);
    let search_query = use_signal::<String>(move || initial_search);
    let search_field = use_signal(warcraft_database::SearchField::default);
    let selected_island = use_signal::<Option<String>>(move || initial_island);
    let selected_hotkey_unit = use_signal::<Option<String>>(move || initial_hotkey_unit);
    let selected_unit_position = use_signal::<Option<String>>(move || initial_unit_position);
    let selected_move_category = use_signal::<Option<String>>(move || initial_move_category);
    let collapsed_categories = use_signal::<HashSet<warcraft_api::UnitKind>>(HashSet::new);
    let show_abilityless_units = use_signal::<bool>(|| false);
    let expand_variants = use_signal::<bool>(|| false);

    let upload_status = use_signal::<UploadStatus>(|| UploadStatus::Idle);
    let mut preview_open = use_signal::<bool>(|| false);
    let mut system_hotkeys_open = use_signal::<bool>(|| false);
    let help_open = use_signal::<bool>(|| !OnboardingPersistence::has_been_seen());
    let layout_dialog_open = use_signal::<bool>(|| false);
    let templates_dialog_open = use_signal::<bool>(|| false);

    let navigator = use_navigator();
    let mut synced_route = use_signal(move || initial_snapshot);
    use_effect(move || {
        if needs_canonicalize {
            navigator.replace(initial_canonical_route.clone());
        }
    });
    let mut search_session_active = use_signal(|| false);
    let mut search_session_gen = use_signal::<u32>(|| 0);
    use_effect(move || {
        let view = *current_view.read();
        let target_snapshot = match view {
            AppView::Editor => {
                let race = *active_race.read();
                let unit_mode_value = *unit_mode.read();
                let selected_unit = selected_unit_id.read().clone();
                let query = search_query.read().clone();
                let nav = DecodedEditorNav {
                    race,
                    unit_mode: unit_mode_value,
                    selected_unit_id: selected_unit,
                    search_query: query,
                };
                NavSnapshot::Editor(nav)
            }
            AppView::Collisions { kind } => {
                let entry = match kind {
                    CollisionKind::Positions => selected_island.read().clone(),
                    CollisionKind::Hotkeys => selected_hotkey_unit.read().clone(),
                    CollisionKind::UnitPositions => selected_unit_position.read().clone(),
                };
                NavSnapshot::Collisions { kind, entry }
            }
            AppView::Resolve => {
                let entry = selected_move_category.read().clone();
                NavSnapshot::Resolve { entry }
            }
        };
        let target_route = Route::from(&target_snapshot);
        let live_snapshot = synced_route.peek().clone();
        let decision = NavDecision::between(&live_snapshot, &target_snapshot);
        let mut clear_session = || {
            if *search_session_active.peek() {
                search_session_active.set(false);
                let next_gen = search_session_gen.peek().wrapping_add(1);
                search_session_gen.set(next_gen);
            }
        };
        match decision {
            NavDecision::Skip => {}
            NavDecision::Push => {
                clear_session();
                navigator.push(target_route);
                synced_route.set(target_snapshot);
            }
            NavDecision::Replace => {
                clear_session();
                navigator.replace(target_route);
                synced_route.set(target_snapshot);
            }
            NavDecision::SessionQuery => {
                let session_was_active = *search_session_active.peek();
                if session_was_active {
                    navigator.replace(target_route);
                } else {
                    navigator.push(target_route);
                    search_session_active.set(true);
                }
                let next_gen = search_session_gen.peek().wrapping_add(1);
                search_session_gen.set(next_gen);
                synced_route.set(target_snapshot);
                spawn(async move {
                    gloo_timers::future::TimeoutFuture::new(500).await;
                    if *search_session_gen.peek() == next_gen {
                        search_session_active.set(false);
                    }
                });
            }
        }
    });

    let handle_keydown = EventHandler::new(move |event: Event<KeyboardData>| {
        let key_value = event.data().key().to_string();
        let shift_held = event.data().modifiers().shift();
        let active_info = FocusedElementInfo::current();
        if key_value == "Tab"
            && active_info
                .as_ref()
                .map(FocusedElementInfo::is_inside_grid_panel)
                .unwrap_or(false)
        {
            event.prevent_default();
            FocusNavigation::cycle_inside_unit_detail(shift_held);
            return;
        }
        if key_value == "Tab"
            && active_info
                .as_ref()
                .map(FocusedElementInfo::is_inside_system_dialog)
                .unwrap_or(false)
        {
            event.prevent_default();
            FocusNavigation::cycle_inside_system_dialog(shift_held);
            return;
        }
        if key_value != "Escape" {
            return;
        }
        if dragging_slot.read().is_some() {
            event.prevent_default();
            dragging_slot.set(None);
            drop_target_tile.set(None);
            drag_follower.set(None);
            return;
        }
        let preview_was_open = *preview_open.read();
        let system_was_open = *system_hotkeys_open.read();
        if system_was_open {
            event.prevent_default();
            system_hotkeys_open.set(false);
            return;
        }
        if preview_was_open {
            event.prevent_default();
            preview_open.set(false);
            return;
        }
        if let Some(info) = active_info {
            let target_selectors: &[&str] = if info.classes().contains("override-key-cell") {
                &[".filled-tile[data-selected=\"true\"]", ".filled-tile"]
            } else if info.classes().contains("filled-tile") {
                &[".unit-card.selected", ".unit-card"]
            } else if info.classes().contains("unit-card")
                || info.classes().contains("unit-category-heading")
            {
                &[".race-tab.active", ".race-tab"]
            } else if info.classes().contains("race-tab") {
                &[".mode-toggle-button.active", ".mode-toggle-button"]
            } else if info.classes().contains("mode-toggle-button") {
                &[".upload-button"]
            } else {
                return;
            };
            if FocusNavigation::first_matching(target_selectors) {
                event.prevent_default();
            }
        }
    });

    let class = style::CLASS;

    let view_navigation = ViewNavigationContext {
        current_view,
        active_race,
        unit_mode,
        selected_unit_id,
        search_query,
    };
    use_context_provider(|| view_navigation);
    let overlay_state = OverlayState {
        preview_open,
        system_hotkeys_open,
        help_open,
        layout_dialog_open,
        templates_dialog_open,
    };
    use_context_provider(|| overlay_state);
    let editor_state = EditorState {
        selected_slot,
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
    };
    use_context_provider(|| editor_state);
    let collision_selection = CollisionSelection {
        selected_island,
        selected_hotkey_unit,
        selected_unit_position,
    };
    use_context_provider(|| collision_selection);
    let resolve_selection = ResolveSelection {
        selected_move_category,
    };
    use_context_provider(|| resolve_selection);
    use_context_provider(|| synced_route);
    use_context_provider(|| loaded_keys);
    use_context_provider(|| grid_layout);
    use_context_provider(|| upload_status);

    ShellModel {
        class,
        handle_keydown,
    }
}
