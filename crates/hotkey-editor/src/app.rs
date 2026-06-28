use std::collections::{HashMap, HashSet};

use dioxus::prelude::*;
use warcraft_api::{Race, UnitKind};
use warcraft_keybinds::CustomKeys;

use crate::components::dialogs::dialog_stack::nested_picker_dialog_is_present;
use crate::components::dialogs::help_dialog::HelpDialog;
use crate::components::dialogs::preview_dialog::PreviewDialog;
use crate::components::shell::footer::Footer;
use crate::components::shell::header::Header;
use crate::components::shell::toasts::ToastMount;
use crate::components::shell::tooltips::TooltipMount;
use crate::components::system_hotkeys::dialog::SystemHotkeysDialog;
use crate::components::tabs::mode_and_race_tabs::ModeAndRaceTabs;
use crate::components::unit_detail::UnitDetailPanel;
use crate::components::unit_list::UnitListPanel;
use crate::components::views::collisions_page::CollisionsPage;
use crate::components::views::resolve_page::ResolvePage;
use crate::model::grid::{DragFollower, DraggingSlot, DropTargetTile};
use crate::services::customkeys::persistence::{CustomKeysPersistence, OnboardingPersistence};
use crate::services::customkeys::upload_status::UploadStatus;
use crate::services::focus::navigation::{FocusNavigation, FocusedElementInfo};
use crate::services::navigation::app_view::{AppView, CollisionKind};
use crate::services::navigation::url_state::UrlNavigationState;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use crate::services::undo::{EditorSnapshot, UndoHistory};
use warcraft_api::RaceLabels;
use warcraft_database::{SearchField, UnitMode};
use warcraft_keybinds::GridSlotId;
use warcraft_keybinds::{GridCoordinate, GridLayout};

const TAILWIND_STYLES: Asset = asset!("/assets/tailwind.css");
const KEYBOARD_NAVIGATION_SCRIPT: Asset = asset!("/assets/keyboard-navigation.js");
const FAVICON: Asset = asset!("/assets/favicon.svg");

/// The history-significant slice of editor navigation state: race, mode,
/// selected unit, and search query. Changing any of these pushes a new browser
/// history entry (so the back button steps through editor selections), whereas
/// an entry-only change (a collision/cascade breadcrumb) merely replaces. Used
/// only to decide push-vs-replace when syncing the URL — it is not reactive.
#[derive(Clone, PartialEq, Eq)]
struct EditorNavKey {
    race: Race,
    unit_mode: UnitMode,
    unit_id: Option<String>,
    query: String,
}

#[component]
pub fn App() -> Element {
    // Boot path: localStorage is the source of truth. If an entry
    // exists, route it through the canonical normalize pipeline; if
    // not, build the default. Either way, write the normalized text
    // back so the entry is always present and ready for the
    // persistence effect below to compare against.
    let loaded_keys = use_signal::<Option<CustomKeys>>(|| {
        let stored_text = CustomKeysPersistence::load_text();
        let initial_file = match stored_text {
            Some(stored) => CustomKeys::from(stored.as_str()).normalize(),
            None => CustomKeys::from("").normalize(),
        };
        let canonical_text = initial_file.to_string();
        CustomKeysPersistence::save_text(&canonical_text);
        Some(initial_file)
    });
    // Persistence: every signal mutation re-runs the canonical
    // pipeline through the facade and writes the normalized text to
    // localStorage. This is the only write path. Mutation sites
    // continue to mutate the in-memory file directly until Phase 4–5
    // of the refactor; the facade ensures whatever they produce is
    // re-normalized before it lands in storage.
    use_effect(move || {
        let read_guard = loaded_keys.read();
        let Some(file) = read_guard.as_ref() else {
            return;
        };
        let normalized = file.normalize();
        let canonical_text = normalized.to_string();
        CustomKeysPersistence::save_text(&canonical_text);
    });
    // Grid layout lives in its own local-storage entry; importing a
    // CustomKeys file or applying a template never touches it, and the
    // layout editor dialog is the only path that mutates it. First-load
    // (no entry yet) falls back to the standard QWERTY layout.
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
    // Undo/redo: one global timeline of full-state snapshots. The capture effect
    // records one entry per committed action (deduped against the present state,
    // so undo/redo restores don't re-record). Provided via context so the
    // toolbar/burger buttons can reach it; Ctrl/Cmd+Z and Ctrl/Cmd+Shift+Z are
    // installed as a window-level shortcut.
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
    // The window keydown listener only writes a request signal (signal reads from
    // outside the Dioxus runtime are unreliable); this reactive effect performs
    // the actual undo/redo where reads/writes are valid.
    use_effect(move || undo_history.handle_keyboard_request());
    let initial_nav = UrlNavigationState::from_url();
    let initial_race = initial_nav.race();
    let initial_mode = initial_nav.unit_mode();
    let initial_unit_id = initial_nav.selected_unit_id().map(|id| id.to_string());
    let initial_search = initial_nav.search_query().to_string();
    let initial_view = initial_nav.view();
    let initial_editor_key = EditorNavKey {
        race: initial_race,
        unit_mode: initial_mode,
        unit_id: initial_unit_id.clone(),
        query: initial_search.clone(),
    };

    // The selected collisions entry is restored into whichever kind's signal the
    // booted view names; the other two start empty (their validation effects
    // pick a first entry on demand).
    let initial_entry = initial_nav.selected_entry().map(|entry| entry.to_string());
    let initial_selected_island = match initial_view {
        AppView::Collisions {
            kind: CollisionKind::Positions,
        } => initial_entry.clone(),
        _ => None,
    };
    let initial_selected_hotkey_unit = match initial_view {
        AppView::Collisions {
            kind: CollisionKind::Hotkeys,
        } => initial_entry.clone(),
        _ => None,
    };
    let initial_selected_unit_position = match initial_view {
        AppView::Collisions {
            kind: CollisionKind::UnitPositions,
        } => initial_entry.clone(),
        _ => None,
    };
    let initial_selected_move_category = match initial_view {
        AppView::Resolve => initial_entry.clone(),
        _ => None,
    };

    let mut active_race = use_signal::<Race>(move || initial_race);
    let mut unit_mode = use_signal::<UnitMode>(move || initial_mode);
    let mut selected_unit_id = use_signal::<Option<String>>(move || initial_unit_id);
    let selected_slot = use_signal::<Option<GridSlotId>>(|| None);
    let selected_from_research = use_signal::<bool>(|| false);
    let selected_from_uprooted = use_signal::<bool>(|| false);
    let hotkey_assign_request = use_signal::<bool>(|| false);
    let tier_overrides = use_signal::<HashMap<String, usize>>(HashMap::new);
    let mut dragging_slot = use_signal::<Option<DraggingSlot>>(|| None);
    let mut drop_target_tile = use_signal::<Option<DropTargetTile>>(|| None);
    let mut drag_follower = use_signal::<Option<DragFollower>>(|| None);
    let editing_layout_cell = use_signal::<Option<GridCoordinate>>(|| None);
    let dragging_layout_cell = use_signal::<Option<GridCoordinate>>(|| None);
    let mut search_query = use_signal::<String>(move || initial_search);
    // What the search query matches against — unit name (default) or ability.
    // The sidebar toggles it; held in memory (not yet a URL param).
    let search_field = use_signal::<SearchField>(SearchField::default);
    let mut current_view = use_signal::<AppView>(move || initial_view);
    let mut selected_island = use_signal::<Option<String>>(move || initial_selected_island);
    let mut selected_hotkey_unit =
        use_signal::<Option<String>>(move || initial_selected_hotkey_unit);
    let mut selected_unit_position =
        use_signal::<Option<String>>(move || initial_selected_unit_position);
    let mut selected_move_category =
        use_signal::<Option<String>>(move || initial_selected_move_category);
    // Tracks the last editor nav key written to the URL so the sync effect can
    // tell a history-significant change (race/mode/unit/search → push) from an
    // entry-only refinement (→ replace). Read via `peek` so the effect does not
    // subscribe to its own writes.
    let mut previous_editor_key = use_signal(move || initial_editor_key);
    // Tracks the last view written to the URL. A view switch is already pushed by
    // ViewNavigationContext::apply, so when the view also changed this effect must
    // not push again (that would double-stack history, e.g. when opening a unit
    // from a collision card moves both the view and the selected unit at once).
    let mut previous_view = use_signal(move || initial_view);
    // Search typing is coalesced into ONE history entry per session: the first
    // query change pushes a boundary entry, subsequent changes only replace it,
    // and the session ends after a short idle so a later search becomes its own
    // entry. This keeps the back button stepping through searches without adding
    // an entry per keystroke. `gen` cancels a stale session-end timer.
    let mut search_session_active = use_signal(|| false);
    let mut search_session_gen = use_signal::<u32>(|| 0);
    use_effect(move || {
        let race = *active_race.read();
        let mode = *unit_mode.read();
        let unit_id_option = selected_unit_id.read().clone();
        let query = search_query.read().clone();
        let view = *current_view.read();
        // Only the active collisions kind's selection rides in the URL.
        let entry_option = match view {
            AppView::Collisions {
                kind: CollisionKind::Positions,
            } => selected_island.read().clone(),
            AppView::Collisions {
                kind: CollisionKind::Hotkeys,
            } => selected_hotkey_unit.read().clone(),
            AppView::Collisions {
                kind: CollisionKind::UnitPositions,
            } => selected_unit_position.read().clone(),
            // On the Resolve view the same `entry` slot carries the selected
            // move-category breadcrumb (Fights/Gap pulls/Spills/Swaps).
            AppView::Resolve => selected_move_category.read().clone(),
            _ => None,
        };
        let unit_id_ref = unit_id_option.as_deref();
        let query_str = query.as_str();
        let entry_ref = entry_option.as_deref();
        // Race/mode/unit/search are history-significant: changing any of them
        // pushes a new entry so the browser back button steps through editor
        // selections. An entry-only change (a collision/cascade breadcrumb) keeps
        // the same editor key and only replaces, so picking entries doesn't flood
        // history. (View switches are pushed by ViewNavigationContext::apply; this
        // effect then sees an unchanged editor key and replaces to refine that
        // just-pushed URL.)
        let current_editor_key = EditorNavKey {
            race,
            unit_mode: mode,
            unit_id: unit_id_option.clone(),
            query: query.clone(),
        };
        let previous_key = previous_editor_key.peek().clone();
        let editor_key_changed = previous_key != current_editor_key;
        let view_changed = *previous_view.peek() != view;
        // A change that touches only the search query (race/mode/unit/view all
        // unchanged) is a search-typing event and is coalesced into one history
        // entry; anything else is a discrete navigation.
        let only_query_changed = !view_changed
            && previous_key.race == race
            && previous_key.unit_mode == mode
            && previous_key.unit_id == unit_id_option
            && previous_key.query != query;
        previous_editor_key.set(current_editor_key);
        previous_view.set(view);
        if only_query_changed {
            let session_was_active = *search_session_active.peek();
            if session_was_active {
                UrlNavigationState::replace_in_url(
                    race,
                    mode,
                    unit_id_ref,
                    query_str,
                    view,
                    entry_ref,
                );
            } else {
                UrlNavigationState::push_view_to_url(
                    race,
                    mode,
                    unit_id_ref,
                    query_str,
                    view,
                    entry_ref,
                );
                search_session_active.set(true);
            }
            let next_gen = search_session_gen.peek().wrapping_add(1);
            search_session_gen.set(next_gen);
            spawn(async move {
                gloo_timers::future::TimeoutFuture::new(500).await;
                if *search_session_gen.peek() == next_gen {
                    search_session_active.set(false);
                }
            });
        } else {
            // Any non-search change ends an in-progress search session.
            if *search_session_active.peek() {
                search_session_active.set(false);
                let next_gen = search_session_gen.peek().wrapping_add(1);
                search_session_gen.set(next_gen);
            }
            if editor_key_changed && !view_changed {
                UrlNavigationState::push_view_to_url(
                    race,
                    mode,
                    unit_id_ref,
                    query_str,
                    view,
                    entry_ref,
                );
            } else {
                UrlNavigationState::replace_in_url(
                    race,
                    mode,
                    unit_id_ref,
                    query_str,
                    view,
                    entry_ref,
                );
            }
        }
    });
    use_hook(move || {
        UrlNavigationState::install_popstate_listener(move |nav_state| {
            let view = nav_state.view();
            current_view.set(view);
            // Race/mode/unit/search are pushed into history, so back/forward must
            // restore them from the popped URL. Sync the push/replace tracker to
            // the restored values too, so the URL-writing effect that re-runs from
            // these signal changes replaces (no new entry) instead of pushing.
            let restored_race = nav_state.race();
            let restored_mode = nav_state.unit_mode();
            let restored_unit_id = nav_state.selected_unit_id().map(|id| id.to_string());
            let restored_query = nav_state.search_query().to_string();
            active_race.set(restored_race);
            unit_mode.set(restored_mode);
            selected_unit_id.set(restored_unit_id.clone());
            search_query.set(restored_query.clone());
            let restored_editor_key = EditorNavKey {
                race: restored_race,
                unit_mode: restored_mode,
                unit_id: restored_unit_id,
                query: restored_query,
            };
            previous_editor_key.set(restored_editor_key);
            previous_view.set(view);
            // A back/forward navigation ends any in-progress search session and
            // cancels its pending session-end timer.
            search_session_active.set(false);
            let next_gen = search_session_gen.peek().wrapping_add(1);
            search_session_gen.set(next_gen);
            // Restore the active kind's selection from the URL; leave the other
            // kinds' in-memory selections untouched (per-tab memory).
            let entry = nav_state.selected_entry().map(|entry| entry.to_string());
            match view {
                AppView::Collisions {
                    kind: CollisionKind::Positions,
                } => selected_island.set(entry),
                AppView::Collisions {
                    kind: CollisionKind::Hotkeys,
                } => selected_hotkey_unit.set(entry),
                AppView::Collisions {
                    kind: CollisionKind::UnitPositions,
                } => selected_unit_position.set(entry),
                AppView::Resolve => selected_move_category.set(entry),
                _ => {}
            }
        });
    });
    let upload_status = use_signal::<UploadStatus>(|| UploadStatus::Idle);
    let mut preview_open = use_signal::<bool>(|| false);
    let mut system_hotkeys_open = use_signal::<bool>(|| false);
    let help_open = use_signal::<bool>(|| !OnboardingPersistence::has_been_seen());
    let collapsed_categories = use_signal::<HashSet<UnitKind>>(HashSet::new);
    let show_abilityless_units = use_signal::<bool>(|| false);
    let expand_variants = use_signal::<bool>(|| false);

    let handle_keydown = move |event: Event<KeyboardData>| {
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

        if nested_picker_dialog_is_present() {
            event.prevent_default();
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
                &[".grid-tile.has-ability.selected", ".grid-tile.has-ability"]
            } else if info.classes().contains("grid-tile") {
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
    };

    // The collisions breadcrumb is the first thing in its view, so it must sit
    // flush under the header divider for its text to centre in the band between
    // that divider and the bar's own border — drop the app's header-to-view gap
    // for this view. The editor and resolve keep the normal gap (their first
    // child is not a bordered bar tied to the divider).
    let is_collisions_view = matches!(*current_view.read(), AppView::Collisions { .. });
    let app_gap_class = if is_collisions_view {
        "gap-0"
    } else {
        "gap-8 max-[2000px]:gap-4 max-[700px]:gap-4 max-[480px]:gap-3"
    };
    let app_class = format!(
        "app mx-auto pt-7 pb-12 px-14 flex flex-col min-h-[100dvh] {app_gap_class} \
         max-[1500px]:pt-0 \
         max-[1024px]:h-auto max-[1024px]:min-h-screen max-[1024px]:overflow-visible \
         max-[700px]:px-4 max-[480px]:px-2"
    );

    rsx! {
        document::Stylesheet { href: TAILWIND_STYLES }
        document::Script { src: KEYBOARD_NAVIGATION_SCRIPT, r#type: "module" }
        document::Link { rel: "icon", r#type: "image/svg+xml", href: FAVICON }
        document::Link { rel: "icon", r#type: "image/x-icon", href: "favicon.ico" }
        document::Link { rel: "apple-touch-icon", href: "icon-192.png" }
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1, viewport-fit=cover",
        }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:title", content: "Warcraft III Hotkey Editor" }
        document::Meta {
            property: "og:description",
            content: "Visual command-card editor for Warcraft III: Reforged. \
                      Drag keys, export CustomKeys.txt — runs entirely in your browser.",
        }
        document::Meta {
            property: "og:image",
            content: "https://clemenscodes.github.io/warcraft-hotkey-editor/og-image.png",
        }
        document::Meta {
            property: "og:url",
            content: "https://clemenscodes.github.io/warcraft-hotkey-editor/",
        }
        document::Meta { name: "twitter:card", content: "summary_large_image" }
        TooltipMount {}
        ToastMount {
            div {
                class: app_class,
            onkeydown: handle_keydown,
            Header {
                loaded_keys,
                upload_status,
                preview_open,
                grid_layout,
                editing_layout_cell,
                dragging_layout_cell,
                update_hotkeys_on_move,
                system_hotkeys_open,
                help_open,
                current_view,
                active_race,
                unit_mode,
                selected_unit_id,
                search_query,
            }
            match *current_view.read() {
                AppView::Editor => rsx! {
                    div {
                        class: "flex items-stretch gap-6 flex-none \
                                min-h-[clamp(9rem,13vh,18rem)] \
                                max-md:flex-col max-md:min-h-0 max-md:gap-[0.85rem]",
                        ModeAndRaceTabs { unit_mode, active_race, selected_unit_id, selected_slot }
                    }
                    div {
                        class: "main-content",
                        "data-race": "{RaceLabels::data_attribute(*active_race.read())}",
                        UnitListPanel { active_race, unit_mode, selected_unit_id, selected_slot, search_query, search_field, show_abilityless_units, expand_variants, collapsed_categories }
                        UnitDetailPanel {
                            active_race,
                            selected_unit_id,
                            selected_slot,
                            selected_from_research,
                            selected_from_uprooted,
                            tier_overrides,
                            dragging_slot,
                            drop_target_tile,
                            drag_follower,
                            loaded_keys,
                            grid_layout,
                            update_hotkeys_on_move,
                            hotkey_assign_request,
                        }
                    }
                },
                AppView::Collisions { kind } => {
                    let view_navigation = ViewNavigationContext {
                        current_view,
                        active_race,
                        unit_mode,
                        selected_unit_id,
                        search_query,
                    };
                    rsx! {
                        CollisionsPage {
                            kind,
                            loaded_keys,
                            grid_layout,
                            view_navigation,
                            selected_island,
                            selected_hotkey_unit,
                            selected_unit_position,
                        }
                    }
                },
                AppView::Resolve => {
                    let view_navigation = ViewNavigationContext {
                        current_view,
                        active_race,
                        unit_mode,
                        selected_unit_id,
                        search_query,
                    };
                    rsx! {
                        ResolvePage { loaded_keys, view_navigation, selected_move_category }
                    }
                },
            }
            Footer {}
                if *preview_open.read() {
                    PreviewDialog { loaded_keys, preview_open }
                }
                if *system_hotkeys_open.read() {
                    SystemHotkeysDialog { loaded_keys, system_hotkeys_open }
                }
                if *help_open.read() {
                    HelpDialog { help_open }
                }
            }
        }
    }
}
