use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};
use warcraft_keybinds::CustomKeys;

use crate::components::dialog_stack::nested_picker_dialog_is_present;
use crate::components::system_hotkeys::control_groups::ControlGroupsHotkeysView;
use crate::components::system_hotkeys::hero_selection::HeroSelectionHotkeysView;
use crate::components::system_hotkeys::inventory::InventoryHotkeysView;
use crate::components::system_hotkeys::inventory_grid::InventoryDragFollower;
use crate::components::system_hotkeys::list_view::SystemHotkeysListView;
use warcraft_database::SystemHotkeysCategory;

const SLOT_FRAME_GOLD: Asset = asset!("/assets/webui/widgets/listitems/list-item-focus-border.png");
const HEADER_GOLD_DECORATION: Asset = asset!("/assets/webui/common/header-decoration-gold.png");

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
    rsx! {
        header {
            class: "relative flex items-center justify-center gap-6 flex-none pt-[1.6rem] px-[4.5rem] pb-[1.4rem] [border-bottom:1px_solid_rgba(255,206,99,0.4)] [box-shadow:0_1px_0_rgba(0,0,0,0.7),0_2px_0_rgba(255,206,99,0.1)] max-[1099px]:[padding:0.85rem_3rem_0.7rem] max-[1099px]:gap-[0.5rem] max-[1099px]:sticky max-[1099px]:top-0 max-[1099px]:z-[5] max-[1099px]:bg-[linear-gradient(135deg,rgba(12,25,50,0.98)_0%,rgba(6,12,28,0.98)_100%)]",
            img {
                class: "h-[2.4rem] w-auto flex-none [filter:drop-shadow(0_1px_0_rgba(0,0,0,0.7))]",
                src: "{HEADER_GOLD_DECORATION}",
                alt: "",
            }
            h2 {
                class: "m-0 font-friz-quadrata uppercase tracking-[0.08em] text-[2.5rem] leading-none text-warcraft-gold [text-shadow:1px_1px_0_#000,0_0_18px_rgba(255,206,99,0.35)] max-[1099px]:text-[clamp(16px,5vw,22px)] max-[1099px]:tracking-[0.04em] max-[1099px]:whitespace-nowrap max-[1099px]:overflow-hidden max-[1099px]:text-ellipsis max-[1099px]:max-w-full",
                "System Hotkeys"
            }
            img {
                class: "h-[2.4rem] w-auto flex-none [filter:drop-shadow(0_1px_0_rgba(0,0,0,0.7))] [transform:scaleX(-1)]",
                src: "{HEADER_GOLD_DECORATION}",
                alt: "",
            }
            button {
                class: "close-button absolute right-4 top-1/2 -translate-y-1/2 w-10 h-10 flex items-center justify-center text-[1.5rem] font-friz-quadrata cursor-pointer transition-[color,text-shadow] duration-150 bg-transparent border-0 text-warcraft-text-secondary [text-shadow:1px_1px_0_#000] hover:text-warcraft-gold hover:[text-shadow:1px_1px_0_#000,0_0_12px_rgba(255,206,99,0.55)] focus:outline-none [body[data-kb-modality]_&]:focus:text-white [body[data-kb-modality]_&]:focus:[text-shadow:1px_1px_0_#000,0_0_16px_rgba(255,255,255,0.7)] max-[1099px]:right-[0.5rem] max-[1099px]:w-[44px] max-[1099px]:h-[44px] max-[1099px]:text-[1.25rem]",
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
    let trigger_label = active.to_string();
    let tab_base = "font-friz-quadrata uppercase tracking-[0.1em] text-[2rem] leading-none \
        px-3 py-1 m-0 bg-transparent border-0 cursor-pointer whitespace-nowrap \
        [text-shadow:1px_1px_0_#000] [transition:color_0.15s_ease,text-shadow_0.15s_ease] \
        hover:text-warcraft-gold hover:[text-shadow:1px_1px_0_#000,0_0_12px_rgba(255,206,99,0.55)] \
        [body[data-kb-modality]_&]:focus:outline-none [body[data-kb-modality]_&]:focus:text-white \
        [body[data-kb-modality]_&]:focus:[text-shadow:1px_1px_0_#000,0_0_14px_rgba(255,255,255,0.65)] \
        group-data-[open=true]:flex-none group-data-[open=true]:w-full group-data-[open=true]:text-left \
        group-data-[open=true]:py-[0.7rem] group-data-[open=true]:px-[0.85rem] \
        group-data-[open=true]:text-[clamp(14px,3.8vw,17px)] group-data-[open=true]:tracking-[0.05em] \
        group-data-[open=true]:min-h-[44px] group-data-[open=true]:rounded-[6px] \
        group-data-[open=true]:whitespace-normal";
    rsx! {
        nav {
            class: "flex items-baseline gap-3 flex-wrap justify-center px-8 py-5 flex-none \
                [border-bottom:1px_solid_rgba(255,206,99,0.25)] \
                max-[1099px]:relative max-[1099px]:flex-nowrap max-[1099px]:justify-stretch \
                max-[1099px]:px-3 max-[1099px]:py-2 max-[1099px]:gap-0 max-[1099px]:overflow-visible",
            aria_label: "System hotkeys categories",
            button {
                class: "hidden max-[1099px]:flex max-[1099px]:items-center max-[1099px]:justify-between \
                    max-[1099px]:w-full max-[1099px]:min-h-[44px] max-[1099px]:py-[0.55rem] \
                    max-[1099px]:px-[0.9rem] \
                    max-[1099px]:bg-[linear-gradient(135deg,rgba(40,30,8,0.85)_0%,rgba(15,12,4,0.85)_100%)] \
                    max-[1099px]:border max-[1099px]:border-[rgba(255,206,99,0.55)] max-[1099px]:rounded-[8px] \
                    max-[1099px]:text-warcraft-gold max-[1099px]:font-friz-quadrata \
                    max-[1099px]:text-[clamp(14px,3.8vw,17px)] max-[1099px]:tracking-[0.06em] \
                    max-[1099px]:uppercase max-[1099px]:[text-shadow:1px_1px_0_rgba(0,0,0,0.92)] \
                    max-[1099px]:[box-shadow:0_0_14px_rgba(255,206,99,0.18)] max-[1099px]:cursor-pointer \
                    [body[data-kb-modality]_&]:focus-visible:outline-none \
                    [body[data-kb-modality]_&]:focus-visible:border-white \
                    [body[data-kb-modality]_&]:focus-visible:text-white \
                    [body[data-kb-modality]_&]:focus-visible:[box-shadow:0_0_0_2px_#fff,0_0_16px_rgba(255,255,255,0.55)]",
                r#type: "button",
                aria_haspopup: "listbox",
                aria_expanded: "{is_open}",
                onclick: move |_| {
                    let next = !*picker_open.read();
                    picker_open.set(next);
                },
                span {
                    class: "max-[1099px]:[flex:1_1_auto] max-[1099px]:text-left \
                        max-[1099px]:whitespace-nowrap max-[1099px]:overflow-hidden \
                        max-[1099px]:text-ellipsis",
                    "{trigger_label}"
                }
                span {
                    class: if is_open {
                        "max-[1099px]:flex-none max-[1099px]:ml-[0.6rem] max-[1099px]:text-[0.9em] \
                        max-[1099px]:leading-none max-[1099px]:[transition:transform_0.18s_ease] rotate-180"
                    } else {
                        "max-[1099px]:flex-none max-[1099px]:ml-[0.6rem] max-[1099px]:text-[0.9em] \
                        max-[1099px]:leading-none max-[1099px]:[transition:transform_0.18s_ease]"
                    },
                    aria_hidden: "true",
                    "\u{25BE}"
                }
            }
            div {
                class: "group flex items-baseline gap-3 flex-wrap justify-center [flex:1_1_auto] \
                    max-[1099px]:hidden \
                    max-[1099px]:data-[open=true]:flex max-[1099px]:data-[open=true]:absolute \
                    max-[1099px]:data-[open=true]:top-[calc(100%-0.25rem)] \
                    max-[1099px]:data-[open=true]:left-3 max-[1099px]:data-[open=true]:right-3 \
                    max-[1099px]:data-[open=true]:z-[6] max-[1099px]:data-[open=true]:flex-col \
                    max-[1099px]:data-[open=true]:items-stretch max-[1099px]:data-[open=true]:gap-[0.15rem] \
                    max-[1099px]:data-[open=true]:p-[0.4rem] \
                    max-[1099px]:data-[open=true]:bg-[linear-gradient(170deg,#0c1d30_0%,#070e1c_100%)] \
                    max-[1099px]:data-[open=true]:border \
                    max-[1099px]:data-[open=true]:border-[rgba(255,206,99,0.45)] \
                    max-[1099px]:data-[open=true]:rounded-[10px] \
                    max-[1099px]:data-[open=true]:[box-shadow:0_14px_30px_rgba(0,0,0,0.7),0_0_18px_rgba(255,206,99,0.12)]",
                role: "listbox",
                "data-open": "{is_open}",
                for (index, category) in SystemHotkeysCategory::ALL.iter().copied().enumerate() {
                    {
                        let is_active = category == active;
                        let tab_color = if is_active {
                            "text-warcraft-gold \
                            [text-shadow:1px_1px_0_#000,0_0_16px_rgba(255,206,99,0.45)] \
                            group-data-[open=true]:bg-[rgba(255,206,99,0.14)] \
                            group-data-[open=true]:[box-shadow:inset_0_0_0_1px_rgba(255,206,99,0.4)]"
                        } else {
                            "text-[rgba(255,206,99,0.55)]"
                        };
                        let tab_class = format!("{tab_base} {tab_color}");
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
                                "{category}"
                            }
                            if let Some(separator_text) = separator {
                                span {
                                    class: "font-friz-quadrata text-[2rem] leading-none select-none \
                                        text-[rgba(255,206,99,0.45)] group-data-[open=true]:hidden",
                                    "{separator_text}"
                                }
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
