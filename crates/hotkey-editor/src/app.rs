use std::collections::{HashMap, HashSet};

use dioxus::prelude::*;
use dioxus_primitives::toast::ToastProvider;
use warcraft_api::{Race, UnitKind};
use warcraft_keybinds::{CustomKeys, CustomKeysFile};

use crate::components::app_footer::AppFooter;
use crate::components::app_header::AppHeader;
use crate::components::dialog_stack::nested_picker_dialog_is_present;
use crate::components::mode_and_race_tabs::ModeAndRaceTabs;
use crate::components::preview_dialog::PreviewDialog;
use crate::components::system_hotkeys::dialog::SystemHotkeysDialog;
use crate::components::unit_detail::UnitDetailPanel;
use crate::components::unit_list::UnitListPanel;
use crate::customkeys::local_storage_cache::LocalStorageCache;
use crate::customkeys::upload_status::UploadStatus;
use crate::domain::grid_layout::{EditingCell, GridLayout};
use crate::domain::grid_slot::{DragFollower, DraggingSlot, DropTargetCell, GridSlotId};
use crate::domain::races::RaceLabels;
use crate::domain::unit_kind::UnitKindHelpers;
use crate::domain::unit_mode::UnitMode;
use crate::focus::navigation::{FocusNavigation, FocusedElementInfo};

const TAILWIND_STYLES: Asset = asset!("/assets/tailwind.css");
const KEYBOARD_NAVIGATION_SCRIPT: Asset = asset!("/assets/keyboard-navigation.js");
const TOOLTIP_TOUCH_SCRIPT: Asset = asset!("/assets/tooltip-touch.js");
const FAVICON: Asset = asset!("/assets/favicon.svg");

#[component]
pub(crate) fn App() -> Element {
    // Boot path: localStorage is the source of truth. If an entry
    // exists, route it through the canonical normalize pipeline; if
    // not, build the default. Either way, write the normalized text
    // back so the entry is always present and ready for the
    // persistence effect below to compare against.
    let loaded_keys = use_signal::<Option<CustomKeysFile>>(|| {
        let stored_text = LocalStorageCache::load_text();
        let initial_custom_keys = match stored_text {
            Some(stored) => CustomKeys::from_text(&stored),
            None => CustomKeys::from_default(),
        };
        LocalStorageCache::save_custom_keys(&initial_custom_keys);
        let canonical_text = initial_custom_keys.to_text();
        let parsed_initial = CustomKeysFile::from(canonical_text);
        Some(parsed_initial)
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
        let custom_keys = CustomKeys::from_file(file);
        LocalStorageCache::save_custom_keys(&custom_keys);
    });
    // Grid layout lives in its own local-storage entry; importing a
    // CustomKeys file or applying a template never touches it, and the
    // layout editor dialog is the only path that mutates it. First-load
    // (no entry yet) falls back to the standard QWERTY layout.
    let grid_layout = use_signal::<GridLayout>(|| {
        LocalStorageCache::load_grid_layout().unwrap_or_else(GridLayout::qwerty_grid)
    });
    use_effect(move || {
        let snapshot = *grid_layout.read();
        LocalStorageCache::save_grid_layout(snapshot);
    });
    let active_race = use_signal::<Race>(|| Race::Human);
    let unit_mode = use_signal::<UnitMode>(|| UnitMode::Melee);
    let selected_unit_id = use_signal::<Option<String>>(|| {
        UnitKindHelpers::default_unit_id_for(Race::Human, UnitMode::Melee)
    });
    let selected_slot = use_signal::<Option<GridSlotId>>(|| None);
    let selected_from_research = use_signal::<bool>(|| false);
    let selected_from_uprooted = use_signal::<bool>(|| false);
    let tier_overrides = use_signal::<HashMap<String, usize>>(HashMap::new);
    let mut dragging_slot = use_signal::<Option<DraggingSlot>>(|| None);
    let mut drop_target_cell = use_signal::<Option<DropTargetCell>>(|| None);
    let mut drag_follower = use_signal::<Option<DragFollower>>(|| None);
    let editing_layout_cell = use_signal::<Option<EditingCell>>(|| None);
    let dragging_layout_cell = use_signal::<Option<EditingCell>>(|| None);
    let search_query = use_signal::<String>(String::new);
    let upload_status = use_signal::<UploadStatus>(|| UploadStatus::Idle);
    let mut preview_open = use_signal::<bool>(|| false);
    let mut system_hotkeys_open = use_signal::<bool>(|| false);
    let collapsed_categories = use_signal::<HashSet<UnitKind>>(HashSet::new);

    rsx! {
        document::Stylesheet { href: TAILWIND_STYLES }
        document::Script { src: KEYBOARD_NAVIGATION_SCRIPT, r#type: "module" }
        document::Script { src: TOOLTIP_TOUCH_SCRIPT, r#type: "module" }
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
        ToastProvider {
            div {
                class: "app mx-auto pt-7 pb-12 px-14 flex flex-col gap-8 \
                        min-h-[100dvh] \
                        max-[2000px]:gap-4 \
                        max-[1500px]:pt-0 \
                        max-[1024px]:h-auto max-[1024px]:min-h-screen max-[1024px]:overflow-visible \
                        max-[700px]:px-4 max-[700px]:gap-4 \
                        max-[480px]:px-2 max-[480px]:gap-3",
            onkeydown: move |event| {
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
                    drop_target_cell.set(None);
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
            },
            AppHeader {
                loaded_keys,
                upload_status,
                preview_open,
                grid_layout,
                editing_layout_cell,
                dragging_layout_cell,
                system_hotkeys_open,
            }
            div {
                class: "flex items-stretch gap-6 flex-none \
                        min-h-[clamp(9rem,13vh,18rem)] \
                        max-md:flex-col max-md:min-h-0 max-md:gap-[0.85rem]",
                ModeAndRaceTabs { unit_mode, active_race, selected_unit_id, selected_slot }
            }
            div {
                class: "main-content",
                "data-race": "{RaceLabels::data_attribute(*active_race.read())}",
                UnitListPanel { active_race, unit_mode, selected_unit_id, selected_slot, search_query, collapsed_categories }
                UnitDetailPanel {
                    selected_unit_id,
                    selected_slot,
                    selected_from_research,
                    selected_from_uprooted,
                    tier_overrides,
                    dragging_slot,
                    drop_target_cell,
                    drag_follower,
                    loaded_keys,
                    grid_layout,
                }
            }
            AppFooter {}
                if *preview_open.read() {
                    PreviewDialog { loaded_keys, preview_open }
                }
                if *system_hotkeys_open.read() {
                    SystemHotkeysDialog { loaded_keys, system_hotkeys_open }
                }
                DragFollowerOverlay { drag_follower, active_race }
            }
        }
    }
}

#[component]
fn DragFollowerOverlay(
    drag_follower: Signal<Option<DragFollower>>,
    active_race: Signal<Race>,
) -> Element {
    let follower_option = drag_follower.read().clone();
    let Some(follower) = follower_option else {
        return rsx! {};
    };
    let visual = follower.visual();
    let style_value = format!(
        "left: {left}px; top: {top}px; width: {width}px; height: {height}px;",
        left = follower.left(),
        top = follower.top(),
        width = follower.tile_width(),
        height = follower.tile_height(),
    );
    let mut class_name = String::from("drag-follower");
    if visual.is_command_cell() {
        class_name.push_str(" is-command");
    }
    let hotkey_overlay_class = if visual.is_passive_command() {
        "hotkey-overlay passive"
    } else {
        "hotkey-overlay"
    };
    let race_attribute = RaceLabels::data_attribute(*active_race.read());
    rsx! {
        div { class: "{class_name}", "data-race": "{race_attribute}", style: "{style_value}",
            if let Some(source) = visual.icon_src() {
                img {
                    src: "{source}",
                    alt: "{visual.label_text()}",
                    draggable: "false",
                    decoding: "async",
                }
            } else {
                span { class: "command-label", "{visual.label_text()}" }
            }
            if let Some(letter_text) = visual.displayed_letter() {
                span { class: "{hotkey_overlay_class}", "{letter_text}" }
            }
        }
    }
}
