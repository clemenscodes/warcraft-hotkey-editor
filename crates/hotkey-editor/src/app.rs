use crate::components::dialogs::help_dialog::HelpDialog;
use crate::components::dialogs::layout_editor::LayoutEditor;
use crate::components::dialogs::preview_dialog::PreviewDialog;
use crate::components::dialogs::system_hotkeys_dialog::SystemHotkeysDialog;
use crate::components::dialogs::templates_dialog::TemplatesDialog;
use crate::components::shell::footer::Footer;
use crate::components::shell::header::Header;
use crate::components::shell::toasts::ToastMount;
use crate::components::shell::tooltips::TooltipMount;
use crate::components::tabs::mode_and_race_tabs::ModeAndRaceTabs;
use crate::components::unit_detail::UnitDetailPanel;
use crate::components::unit_list::UnitList;
use crate::components::views::collisions_page::CollisionsPage;
use crate::components::views::resolve_page::ResolvePage;
use crate::model::grid::{DragFollower, DraggingSlot, DropTargetTile};
use crate::services::customkeys::persistence::{CustomKeysPersistence, OnboardingPersistence};
use crate::services::customkeys::upload_status::UploadStatus;
use crate::services::focus::navigation::{FocusNavigation, FocusedElementInfo};
use crate::services::navigation::app_view::{AppView, CollisionKind};
use crate::services::navigation::url_state::UrlNavigationState;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use crate::services::overlay_state::OverlayState;
use crate::services::undo::{EditorSnapshot, UndoHistory};
use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};
use warcraft_api::RaceLabels;
use warcraft_api::{Race, UnitKind};
use warcraft_database::{SearchField, UnitMode};
use warcraft_keybinds::CustomKeys;
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
    use_effect(move || {
        let read_guard = loaded_keys.read();
        let Some(file) = read_guard.as_ref() else {
            return;
        };
        let normalized = file.normalize();
        let canonical_text = normalized.to_string();
        CustomKeysPersistence::save_text(&canonical_text);
    });
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
    let search_field = use_signal::<SearchField>(SearchField::default);
    let mut current_view = use_signal::<AppView>(move || initial_view);
    let mut selected_island = use_signal::<Option<String>>(move || initial_selected_island);
    let mut selected_hotkey_unit =
        use_signal::<Option<String>>(move || initial_selected_hotkey_unit);
    let mut selected_unit_position =
        use_signal::<Option<String>>(move || initial_selected_unit_position);
    let mut selected_move_category =
        use_signal::<Option<String>>(move || initial_selected_move_category);
    let mut previous_editor_key = use_signal(move || initial_editor_key);
    let mut previous_view = use_signal(move || initial_view);
    let mut search_session_active = use_signal(|| false);
    let mut search_session_gen = use_signal::<u32>(|| 0);
    use_effect(move || {
        let race = *active_race.read();
        let mode = *unit_mode.read();
        let unit_id_option = selected_unit_id.read().clone();
        let query = search_query.read().clone();
        let view = *current_view.read();
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
            AppView::Resolve => selected_move_category.read().clone(),
            _ => None,
        };
        let unit_id_ref = unit_id_option.as_deref();
        let query_str = query.as_str();
        let entry_ref = entry_option.as_deref();
        let current_editor_key = EditorNavKey {
            race,
            unit_mode: mode,
            unit_id: unit_id_option.clone(),
            query: query.clone(),
        };
        let previous_key = previous_editor_key.peek().clone();
        let editor_key_changed = previous_key != current_editor_key;
        let view_changed = *previous_view.peek() != view;
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
            search_session_active.set(false);
            let next_gen = search_session_gen.peek().wrapping_add(1);
            search_session_gen.set(next_gen);
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
    let layout_dialog_open = use_signal::<bool>(|| false);
    let templates_dialog_open = use_signal::<bool>(|| false);
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
    };
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
         max-[700px]:px-4 max-[480px]:px-2",
    );
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
            div { class: app_class, onkeydown: handle_keydown,
                Header { loaded_keys, upload_status, grid_layout }
                match *current_view.read() {
                    AppView::Editor => rsx! {
                        div { class: "flex items-stretch gap-6 flex-none \
                                                        min-h-[clamp(9rem,13vh,18rem)] \
                                                        max-md:flex-col max-md:min-h-0 max-md:gap-[0.85rem]",
                            ModeAndRaceTabs {
                                unit_mode,
                                active_race,
                                selected_unit_id,
                                selected_slot,
                            }
                        }
                        div {
                            class: "main-content",
                            "data-race": "{RaceLabels::data_attribute(*active_race.read())}",
                            UnitList {
                                active_race,
                                unit_mode,
                                selected_unit_id,
                                selected_slot,
                                search_query,
                                search_field,
                                show_abilityless_units,
                                expand_variants,
                                collapsed_categories,
                            }
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
                    AppView::Collisions { kind } => rsx! {
                        CollisionsPage {
                            kind,
                            loaded_keys,
                            grid_layout,
                            selected_island,
                            selected_hotkey_unit,
                            selected_unit_position,
                        }
                    },
                    AppView::Resolve => rsx! {
                        ResolvePage { loaded_keys, selected_move_category }
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
                TemplatesDialog {
                    loaded_keys,
                    upload_status,
                    open: templates_dialog_open,
                }
                LayoutEditor {
                    grid_layout,
                    editing_layout_cell,
                    dragging_layout_cell,
                    update_hotkeys_on_move,
                    loaded_keys,
                    open: layout_dialog_open,
                }
            }
        }
    }
}
