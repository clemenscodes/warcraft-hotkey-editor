use crate::components::app::components::shell::logic::RouteBootstrap;
use crate::components::app::components::shell::route_sync::NavDecision;
use crate::components::app::route::Route;
use crate::persistence::custom_keys_persistence::CustomKeysPersistence;
use crate::persistence::editor_preferences_persistence::EditorPreferencesPersistence;
use crate::persistence::grid_layout_persistence::GridLayoutPersistence;
use crate::persistence::onboarding_persistence::OnboardingPersistence;
use crate::services::collision_selection::CollisionSelection;
use crate::services::customkeys::service::CustomKeysService;
use crate::services::customkeys::upload_status::UploadStatus;
use crate::services::editor_state::EditorState;
use crate::services::editor_state::{DragFollower, DraggingSlot, DropTargetTile};
use crate::services::grid_layout::service::GridLayoutService;
use crate::services::navigation::app_view::{AppView, CollisionKind};
use crate::services::navigation::editor_nav::DecodedEditorNav;
use crate::services::navigation::nav_snapshot::NavSnapshot;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use crate::services::overlay_state::OverlayState;
use crate::services::resolve_selection::ResolveSelection;
use crate::services::undo::UndoHistory;
use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::EditorSnapshot;
use warcraft_keybinds::GridLayout;
use warcraft_keybinds::GridSlotId;

/// The one thing the [`Shell`](super::Shell) body needs beyond its own class: the
/// app-level key handler. Every piece of app-wide state the shell owns is handed to
/// the page tree, the header, and the header's dialog hosts through context, so the
/// body is a flat list of children — no god-bag of props.
pub(super) struct ShellModel {
    pub(super) handle_keydown: EventHandler<KeyboardEvent>,
}

/// The app-wide signals the shell owns, grouped by the context they populate. Held
/// together so the URL-sync effect and the key handler can read the exact slices they
/// need after every signal is created and provided.
struct AppSignals {
    view_navigation: ViewNavigationContext,
    collision_selection: CollisionSelection,
    resolve_selection: ResolveSelection,
    synced_route: Signal<NavSnapshot>,
    dragging_slot: Signal<Option<DraggingSlot>>,
    drop_target_tile: Signal<Option<DropTargetTile>>,
    drag_follower: Signal<Option<DragFollower>>,
    preview_open: Signal<bool>,
    system_hotkeys_open: Signal<bool>,
}

/// Load the canonical document from storage into a signal, provide it and its service
/// as context, and persist every change back. localStorage is the source of truth: the
/// signal is a read cache re-serialized to storage on every write.
fn use_custom_keys_document() -> Signal<Option<CustomKeys>> {
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
        // Invariant: every writer of `loaded_keys` stores a normalized aggregate
        // (commit/import normalize, from_text normalizes, resolve-Apply normalizes,
        // template-apply uses import_overlay's normalized output). So re-normalizing
        // here is redundant work; just serialize. The debug assertion catches any
        // future writer that violates the invariant.
        debug_assert_eq!(
            file.clone().normalize().to_string(),
            file.to_string(),
            "loaded_keys held a non-normalized aggregate; a writer must normalize before set()",
        );
        let canonical_text = file.to_string();
        CustomKeysPersistence::save_text(&canonical_text);
    });
    let custom_keys_service = CustomKeysService::new(loaded_keys);
    use_context_provider(|| custom_keys_service);
    use_context_provider(|| loaded_keys);
    loaded_keys
}

/// Load the grid layout from storage into a signal, provide it and its service as
/// context, and persist every change back.
fn use_grid_layout_document() -> Signal<GridLayout> {
    let grid_layout = use_signal::<GridLayout>(|| {
        GridLayoutPersistence::load_grid_layout().unwrap_or_else(GridLayout::qwerty_grid)
    });
    use_effect(move || {
        let snapshot = *grid_layout.read();
        GridLayoutPersistence::save_grid_layout(snapshot);
    });
    let grid_layout_service = GridLayoutService::new(grid_layout);
    use_context_provider(|| grid_layout_service);
    use_context_provider(|| grid_layout);
    grid_layout
}

/// The "update hotkeys as bindings move" editor preference, loaded from storage and
/// persisted on change.
fn use_editor_preferences() -> Signal<bool> {
    let update_hotkeys_on_move =
        use_signal::<bool>(EditorPreferencesPersistence::load_update_hotkeys_on_move);
    use_effect(move || {
        let enabled = *update_hotkeys_on_move.read();
        EditorPreferencesPersistence::save_update_hotkeys_on_move(enabled);
    });
    update_hotkeys_on_move
}

/// Wire the undo/redo history: build it over the document and layout signals, provide
/// it as context, record a snapshot on every change, and install its keyboard
/// shortcuts.
fn use_editor_history(loaded_keys: Signal<Option<CustomKeys>>, grid_layout: Signal<GridLayout>) {
    let undo_history = UndoHistory::use_history(loaded_keys, grid_layout);
    use_context_provider(|| undo_history);
    use_effect(move || {
        let keys_text = loaded_keys
            .read()
            .as_ref()
            .map(|file| file.to_string())
            .unwrap_or_default();
        let grid_layout_text = grid_layout.read().to_storage_string();
        let snapshot = EditorSnapshot::new(keys_text, grid_layout_text);
        undo_history.record(snapshot);
    });
    use_hook(move || undo_history.install_keyboard_shortcuts());
    use_effect(move || undo_history.handle_keyboard_request());
}

/// Decode the entry URL once into the shell's opening state, and canonicalize the
/// address bar on entry if the URL was bare or partial. The signal→URL sync never
/// fires here, since the decoded state already matches, so this gets its own one-time
/// replace.
fn use_route_bootstrap() -> RouteBootstrap {
    let initial_route = router().current::<Route>();
    let bootstrap = RouteBootstrap::from(&initial_route);
    let navigator = use_navigator();
    let needs_canonicalize = bootstrap.needs_canonicalize;
    let canonical_route = bootstrap.canonical_route.clone();
    use_effect(move || {
        if needs_canonicalize {
            navigator.replace(canonical_route.clone());
        }
    });
    bootstrap
}

/// Create every app-wide signal seeded from the route bootstrap, bundle them into the
/// context structs the pages and header read, provide those, and return the slices the
/// URL-sync effect and key handler still need directly.
fn use_app_signals(bootstrap: RouteBootstrap, update_hotkeys_on_move: Signal<bool>) -> AppSignals {
    let RouteBootstrap {
        snapshot,
        view,
        nav,
        selected_island,
        selected_hotkey_unit,
        selected_unit_position,
        selected_move_category,
        ..
    } = bootstrap;
    let initial_view = view;
    let initial_race = nav.race();
    let initial_mode = nav.unit_mode();
    let initial_unit_id = nav.selected_unit_id();
    let initial_search = nav.search_query().to_owned();
    let initial_island = selected_island;
    let initial_hotkey_unit = selected_hotkey_unit;
    let initial_unit_position = selected_unit_position;
    let initial_move_category = selected_move_category;
    let initial_snapshot = snapshot;

    let current_view = use_signal::<AppView>(move || initial_view);
    let active_race = use_signal(move || initial_race);
    let unit_mode = use_signal(move || initial_mode);
    let selected_unit_id = use_signal::<Option<WarcraftObjectId>>(move || initial_unit_id);
    let selected_slot = use_signal::<Option<GridSlotId>>(|| None);
    let selected_from_research = use_signal::<bool>(|| false);
    let selected_from_uprooted = use_signal::<bool>(|| false);
    let hotkey_assign_request = use_signal::<bool>(|| false);
    let tier_overrides = use_signal::<HashMap<WarcraftObjectId, usize>>(HashMap::new);
    let dragging_slot = use_signal::<Option<DraggingSlot>>(|| None);
    let drop_target_tile = use_signal::<Option<DropTargetTile>>(|| None);
    let drag_follower = use_signal::<Option<DragFollower>>(|| None);
    let search_query = use_signal::<String>(move || initial_search);
    let search_field = use_signal(warcraft_api::SearchField::default);
    let selected_island = use_signal::<Option<String>>(move || initial_island);
    let selected_hotkey_unit = use_signal::<Option<String>>(move || initial_hotkey_unit);
    let selected_unit_position = use_signal::<Option<String>>(move || initial_unit_position);
    let selected_move_category = use_signal::<Option<String>>(move || initial_move_category);
    let collapsed_categories = use_signal::<HashSet<warcraft_api::UnitKind>>(HashSet::new);
    let active_category = use_signal::<warcraft_api::UnitKind>(|| warcraft_api::UnitKind::Soldier);
    let show_abilityless_units = use_signal::<bool>(|| false);
    let expand_variants = use_signal::<bool>(|| false);
    let upload_status = use_signal::<UploadStatus>(|| UploadStatus::Idle);
    let selected_hero_level = use_signal::<u32>(|| 1);
    let preview_open = use_signal::<bool>(|| false);
    let system_hotkeys_open = use_signal::<bool>(|| false);
    let help_open = use_signal::<bool>(|| !OnboardingPersistence::has_been_seen());
    let layout_dialog_open = use_signal::<bool>(|| false);
    let templates_dialog_open = use_signal::<bool>(|| false);
    let synced_route = use_signal(move || initial_snapshot);

    let view_navigation = ViewNavigationContext::new(
        current_view,
        active_race,
        unit_mode,
        selected_unit_id,
        search_query,
    );
    use_context_provider(|| view_navigation);
    let overlay_state = OverlayState::new(
        preview_open,
        system_hotkeys_open,
        help_open,
        layout_dialog_open,
        templates_dialog_open,
    );
    use_context_provider(|| overlay_state);
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
    let collision_selection = CollisionSelection::new(
        selected_island,
        selected_hotkey_unit,
        selected_unit_position,
    );
    use_context_provider(|| collision_selection);
    let resolve_selection = ResolveSelection::new(selected_move_category);
    use_context_provider(|| resolve_selection);
    use_context_provider(|| synced_route);
    use_context_provider(|| upload_status);

    AppSignals {
        view_navigation,
        collision_selection,
        resolve_selection,
        synced_route,
        dragging_slot,
        drop_target_tile,
        drag_follower,
        preview_open,
        system_hotkeys_open,
    }
}

/// The write side of the URL contract: on any state change, build the route the
/// signals now describe and diff it against the address bar, pushing a new history
/// entry for a page/selection change, replacing for an entry pick, and running the
/// debounced search-typing session for a query edit. Comparing against the live route
/// (read without subscribing) is what keeps a browser back/forward from being echoed
/// straight back.
fn use_url_sync(signals: &AppSignals) {
    let current_view = signals.view_navigation.current_view();
    let active_race = signals.view_navigation.active_race();
    let unit_mode = signals.view_navigation.unit_mode();
    let selected_unit_id = signals.view_navigation.selected_unit_id();
    let search_query = signals.view_navigation.search_query();
    let selected_island = signals.collision_selection.selected_island();
    let selected_hotkey_unit = signals.collision_selection.selected_hotkey_unit();
    let selected_unit_position = signals.collision_selection.selected_unit_position();
    let selected_move_category = signals.resolve_selection.selected_move_category();
    let mut synced_route = signals.synced_route;
    let navigator = use_navigator();
    let mut search_session_active = use_signal(|| false);
    let mut search_session_gen = use_signal::<u32>(|| 0);
    use_effect(move || {
        let view = *current_view.read();
        let target_snapshot = match view {
            AppView::Editor => {
                let race = *active_race.read();
                let unit_mode_value = *unit_mode.read();
                let selected_unit = *selected_unit_id.read();
                let query = search_query.read().clone();
                let nav = DecodedEditorNav::new(race, unit_mode_value, selected_unit, query);
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
}

/// The app-level Escape handler: cancels an in-progress drag, otherwise closes the top
/// open overlay. All keyboard focus movement is the browser's native Tab order.
fn use_app_keydown(signals: &AppSignals) -> EventHandler<KeyboardEvent> {
    let mut dragging_slot = signals.dragging_slot;
    let mut drop_target_tile = signals.drop_target_tile;
    let mut drag_follower = signals.drag_follower;
    let mut preview_open = signals.preview_open;
    let mut system_hotkeys_open = signals.system_hotkeys_open;
    EventHandler::new(move |event: Event<KeyboardData>| {
        let key_value = event.data().key().to_string();
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
        }
    })
}

/// Build the app shell's full model: load the document and grid layout from storage,
/// wire the persistence/undo effects, own every app-wide signal, provide the contexts
/// the header and the routed pages read, and run the URL-sync push effect. Each concern
/// is its own sub-hook; the body composes them and returns the two things the body
/// renders.
pub(super) fn use_shell() -> ShellModel {
    let loaded_keys = use_custom_keys_document();
    let grid_layout = use_grid_layout_document();
    let update_hotkeys_on_move = use_editor_preferences();
    use_editor_history(loaded_keys, grid_layout);
    let bootstrap = use_route_bootstrap();
    let signals = use_app_signals(bootstrap, update_hotkeys_on_move);
    use_url_sync(&signals);
    let handle_keydown = use_app_keydown(&signals);
    ShellModel { handle_keydown }
}
