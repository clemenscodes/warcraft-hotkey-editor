mod route_snapshot_codec;

use crate::components::app::route::Route;
use crate::persistence::custom_keys_persistence;
use crate::persistence::editor_preferences_persistence;
use crate::persistence::grid_layout_persistence;
use crate::services::collision_selection::context::use_collision_selection_provider;
use crate::services::customkeys::service::CustomKeysService;
use crate::services::customkeys::upload_status::UploadStatus;
use crate::services::drag_state::DragState;
use crate::services::drag_state::context::use_drag_state_provider;
use crate::services::editor_state::context::use_editor_state_provider;
use crate::services::grid_layout::service::GridLayoutService;
use crate::services::navigation::app_view::AppView;
use crate::services::navigation::app_view::CollisionKind;
use crate::services::navigation::context::use_view_navigation_provider;
use crate::services::navigation::editor_navigation::DecodedEditorNavigation;
use crate::services::navigation::navigation_command::{NavigationCommand, NavigationHistoryMode};
use crate::services::navigation::navigation_snapshot::NavigationSnapshot;
use crate::services::overlay_state::OverlayState;
use crate::services::overlay_state::context::use_overlay_state_provider;
use crate::services::resolve_selection::context::use_resolve_selection_provider;
use crate::services::undo::UndoHistory;
use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::EditorSnapshot;
use warcraft_keybinds::GridLayout;

/// The app's opening state, decoded once from the entry URL: the canonical route the
/// address bar should show, whether the entry URL needs canonicalizing, and every
/// initial value the shell's signals seed from.
///
/// A bare or partial URL (`/`, `/collisions`) decodes to the same state as its
/// fully-materialized form (`/?race=human&mode=melee&unit=…`); `canonical_route` is
/// that materialized form and `needs_canonicalize` is whether the entry differed, so
/// the shell can replace the address bar once on entry.
#[derive(Clone)]
pub(super) struct RouteBootstrap {
    pub(super) canonical_route: Route,
    pub(super) needs_canonicalize: bool,
    pub(super) view: AppView,
    pub(super) navigation: DecodedEditorNavigation,
    pub(super) selected_island: Option<String>,
    pub(super) selected_hotkey_unit: Option<String>,
    pub(super) selected_unit_position: Option<String>,
    pub(super) selected_move_category: Option<String>,
}

impl From<&Route> for RouteBootstrap {
    fn from(initial_route: &Route) -> Self {
        let snapshot = NavigationSnapshot::from(initial_route);
        let canonical_route = Route::from(&snapshot);
        let needs_canonicalize = *initial_route != canonical_route;
        let view = match &snapshot {
            NavigationSnapshot::Editor(_) => AppView::Editor,
            NavigationSnapshot::Collisions { kind, .. } => AppView::Collisions { kind: *kind },
            NavigationSnapshot::Resolve { .. } => AppView::Resolve,
        };
        let navigation = match &snapshot {
            NavigationSnapshot::Editor(navigation) => navigation.clone(),
            _ => DecodedEditorNavigation::decode(None, None, None, None),
        };
        let selected_island = match &snapshot {
            NavigationSnapshot::Collisions {
                kind: CollisionKind::Positions,
                entry,
            } => entry.clone(),
            _ => None,
        };
        let selected_hotkey_unit = match &snapshot {
            NavigationSnapshot::Collisions {
                kind: CollisionKind::Hotkeys,
                entry,
            } => entry.clone(),
            _ => None,
        };
        let selected_unit_position = match &snapshot {
            NavigationSnapshot::Collisions {
                kind: CollisionKind::UnitPositions,
                entry,
            } => entry.clone(),
            _ => None,
        };
        let selected_move_category = match &snapshot {
            NavigationSnapshot::Resolve { entry } => entry.clone(),
            _ => None,
        };
        Self {
            canonical_route,
            needs_canonicalize,
            view,
            navigation,
            selected_island,
            selected_hotkey_unit,
            selected_unit_position,
            selected_move_category,
        }
    }
}

/// The one thing the [`Shell`](super::Shell) body needs beyond its own class: the
/// app-level key handler. Every piece of app-wide state the shell owns is handed to
/// the page tree, the header, and the header's dialog hosts through context, so the
/// body is a flat list of children — no god-bag of props.
pub(super) struct ShellModel {
    pub(super) handle_keydown: EventHandler<KeyboardEvent>,
}

/// Load the canonical document from storage into a signal, provide it and its service
/// as context, and persist every change back. localStorage is the source of truth: the
/// signal is a read cache re-serialized to storage on every write.
fn use_custom_keys_document() -> Signal<Option<CustomKeys>> {
    let loaded_keys = use_signal::<Option<CustomKeys>>(|| {
        let stored_text = custom_keys_persistence::load_text();
        let initial_file = match stored_text {
            Some(stored) => CustomKeys::from_text(stored.as_str()),
            None => CustomKeys::from_text(""),
        };
        let canonical_text = initial_file.to_string();
        custom_keys_persistence::save_text(&canonical_text);
        Some(initial_file)
    });
    use_effect(move || {
        let read_guard = loaded_keys.read();
        let Some(file) = read_guard.as_ref() else {
            return;
        };
        let canonical_text = file.to_string();
        custom_keys_persistence::save_text(&canonical_text);
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
        grid_layout_persistence::load_grid_layout().unwrap_or_else(GridLayout::qwerty_grid)
    });
    use_effect(move || {
        let snapshot = *grid_layout.read();
        grid_layout_persistence::save_grid_layout(snapshot);
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
        use_signal(editor_preferences_persistence::load_update_hotkeys_on_move);
    use_effect(move || {
        let enabled = *update_hotkeys_on_move.read();
        editor_preferences_persistence::save_update_hotkeys_on_move(enabled);
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
/// address bar on entry if the URL was bare or partial. No signals→route effect exists
/// to materialize a bare URL, so this one-time replace is the sole home for that.
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

/// The component-layer navigation seam: the callback the navigation service invokes with
/// a typed [`NavigationCommand`]. This is the only place that names the concrete `Route`
/// — it turns the command's snapshot into a route and pushes or replaces it — so the
/// `services/navigation` layer stays route-agnostic.
fn use_navigation_dispatch() -> Callback<NavigationCommand> {
    let navigator = use_navigator();
    use_callback(move |command: NavigationCommand| {
        let snapshot = command.snapshot();
        let route = Route::from(snapshot);
        match command.history_mode() {
            NavigationHistoryMode::Push => {
                navigator.push(route);
            }
            NavigationHistoryMode::Replace => {
                navigator.replace(route);
            }
        }
    })
}

/// The app-level Escape handler: cancels an in-progress drag, otherwise closes the top
/// open overlay. All keyboard focus movement is the browser's native Tab order.
fn use_app_keydown(
    drag_state: DragState,
    overlay_state: OverlayState,
) -> EventHandler<KeyboardEvent> {
    let mut dragging_slot = drag_state.dragging_slot();
    let mut drop_target_tile = drag_state.drop_target_tile();
    let mut drag_follower = drag_state.drag_follower();
    let mut preview_open = overlay_state.preview_open();
    let mut system_hotkeys_open = overlay_state.system_hotkeys_open();
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
/// the header and the routed pages read, and hand back the app-level key handler. Each
/// concern is its own sub-hook. There is no signals→route effect: the write side of the
/// URL contract lives at each mutation site (via the navigation service), and each page
/// reconciles the route back into these signals on the read side.
pub(super) fn use_shell() -> ShellModel {
    let loaded_keys = use_custom_keys_document();
    let grid_layout = use_grid_layout_document();
    let update_hotkeys_on_move = use_editor_preferences();
    use_editor_history(loaded_keys, grid_layout);
    let bootstrap = use_route_bootstrap();
    let RouteBootstrap {
        view,
        navigation,
        selected_island,
        selected_hotkey_unit,
        selected_unit_position,
        selected_move_category,
        ..
    } = bootstrap;
    let collision_selection = use_collision_selection_provider(
        selected_island,
        selected_hotkey_unit,
        selected_unit_position,
    );
    let resolve_selection = use_resolve_selection_provider(selected_move_category);
    let navigation_dispatch = use_navigation_dispatch();
    use_view_navigation_provider(
        view,
        navigation,
        collision_selection,
        resolve_selection,
        navigation_dispatch,
    );
    let overlay_state = use_overlay_state_provider();
    use_editor_state_provider(update_hotkeys_on_move);
    let drag_state = use_drag_state_provider();
    let upload_status = use_signal::<UploadStatus>(|| UploadStatus::Idle);
    use_context_provider(|| upload_status);
    let handle_keydown = use_app_keydown(drag_state, overlay_state);
    ShellModel { handle_keydown }
}
