use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};
use warcraft_keybinds::CustomKeys;

use crate::components::dialog_stack::nested_picker_dialog_is_present;
use crate::components::system_hotkeys::control_groups::ControlGroupsHotkeysView;
use crate::components::system_hotkeys::hero_selection::HeroSelectionHotkeysView;
use crate::components::system_hotkeys::inventory::InventoryHotkeysView;
use crate::components::system_hotkeys::inventory_grid::InventoryDragFollower;
use crate::components::system_hotkeys::list_view::SystemHotkeysListView;
use crate::system_hotkeys::category::SystemHotkeysCategory;

const HEADER_GOLD_DECORATION: Asset = asset!("/assets/webui/common/header-decoration-gold.png");
const SLOT_FRAME_GOLD: Asset = asset!("/assets/webui/widgets/listitems/list-item-focus-border.png");

#[component]
pub(crate) fn SystemHotkeysDialog(
    loaded_keys: Signal<Option<CustomKeys>>,
    mut system_hotkeys_open: Signal<bool>,
) -> Element {
    let editing_section = use_signal::<Option<String>>(|| None);
    let active_category = use_signal::<SystemHotkeysCategory>(|| SystemHotkeysCategory::Inventory);
    let drag_follower = use_signal::<Option<InventoryDragFollower>>(|| None);
    let active = *active_category.read();

    rsx! {
        DialogRoot {
            class: "dialog-overlay",
            open: system_hotkeys_open(),
            on_open_change: move |is_open: bool| {
                if !is_open && nested_picker_dialog_is_present() {
                    return;
                }
                system_hotkeys_open.set(is_open);
            },
            DialogContent { class: "dialog-shell wc3-dialog system-hotkeys-dialog".to_string(),
                SystemHotkeysHeader {
                    on_close: move |_| system_hotkeys_open.set(false),
                }
                SystemHotkeysBreadcrumbs { active_category }
                div { class: "wc3-dialog-body",
                    match active {
                        SystemHotkeysCategory::Inventory => rsx! {
                            InventoryHotkeysView { loaded_keys, editing_section, drag_follower }
                        },
                        SystemHotkeysCategory::HeroSelection => rsx! {
                            HeroSelectionHotkeysView { loaded_keys, editing_section }
                        },
                        SystemHotkeysCategory::ControlGroups => rsx! {
                            ControlGroupsHotkeysView { loaded_keys, editing_section }
                        },
                        other_category => rsx! {
                            SystemHotkeysListView { category: other_category, loaded_keys, editing_section }
                        },
                    }
                }
                InventoryDragOverlay { drag_follower }
            }
        }
    }
}

#[component]
fn SystemHotkeysHeader(on_close: EventHandler<()>) -> Element {
    let gold_decoration_url = HEADER_GOLD_DECORATION;
    rsx! {
        header { class: "wc3-dialog-header",
            img { class: "wc3-header-decoration", src: "{gold_decoration_url}", alt: "" }
            h2 { class: "wc3-dialog-title", "System Hotkeys" }
            img { class: "wc3-header-decoration wc3-header-decoration-mirrored", src: "{gold_decoration_url}", alt: "" }
            button {
                class: "wc3-close-button",
                r#type: "button",
                aria_label: "Close",
                onclick: move |_| on_close.call(()),
                "\u{2715}"
            }
        }
    }
}

#[component]
fn SystemHotkeysBreadcrumbs(mut active_category: Signal<SystemHotkeysCategory>) -> Element {
    let active = *active_category.read();
    let mut picker_open = use_signal::<bool>(|| false);
    let is_open = picker_open();
    let trigger_label = active.label();
    rsx! {
        nav { class: "wc3-breadcrumbs", aria_label: "System hotkeys categories",
            button {
                class: "wc3-breadcrumbs-trigger",
                r#type: "button",
                aria_haspopup: "listbox",
                aria_expanded: "{is_open}",
                onclick: move |_| {
                    let next = !*picker_open.read();
                    picker_open.set(next);
                },
                span { class: "wc3-breadcrumbs-trigger-label", "{trigger_label}" }
                span {
                    class: "wc3-breadcrumbs-trigger-chevron",
                    aria_hidden: "true",
                    "\u{25BE}"
                }
            }
            div {
                class: "wc3-breadcrumbs-list",
                role: "listbox",
                "data-open": "{is_open}",
                for (index, category) in SystemHotkeysCategory::ALL.iter().copied().enumerate() {
                    {
                        let is_active = category == active;
                        let mut tab_class = String::from("wc3-breadcrumb-tab");
                        if is_active {
                            tab_class.push_str(" active");
                        }
                        let separator = if index + 1 < SystemHotkeysCategory::ALL.len() {
                            Some("›")
                        } else {
                            None
                        };
                        rsx! {
                            button {
                                class: "{tab_class}",
                                r#type: "button",
                                role: "option",
                                aria_selected: "{is_active}",
                                aria_current: if is_active { "page" } else { "false" },
                                onclick: move |_| {
                                    active_category.set(category);
                                    picker_open.set(false);
                                },
                                "{category.label()}"
                            }
                            if let Some(separator_text) = separator {
                                span { class: "wc3-breadcrumb-separator", "{separator_text}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn InventoryDragOverlay(drag_follower: Signal<Option<InventoryDragFollower>>) -> Element {
    let follower_option = drag_follower.read().clone();
    let Some(follower) = follower_option else {
        return rsx! {};
    };
    let frame_url = SLOT_FRAME_GOLD;
    let style_value = format!(
        "left: {left}px; top: {top}px; width: {width}px; height: {height}px; \
         --wc3-slot-frame: url('{frame_url}');",
        left = follower.left(),
        top = follower.top(),
        width = follower.width(),
        height = follower.height(),
    );
    let label_text = follower.label().to_string();
    rsx! {
        div { class: "wc3-inventory-drag-follower", style: "{style_value}",
            div { class: "wc3-slot-key", "{label_text}" }
        }
    }
}
