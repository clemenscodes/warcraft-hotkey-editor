use dioxus::prelude::*;
use warcraft_database::SystemHotkeysCategory;

use super::category_tab::SystemHotkeysCategoryTab;

#[derive(Props, Clone, PartialEq)]
pub(super) struct SystemHotkeysBreadcrumbsProps {
    pub(super) active_category: Signal<SystemHotkeysCategory>,
}

#[component]
pub(super) fn SystemHotkeysBreadcrumbs(props: SystemHotkeysBreadcrumbsProps) -> Element {
    let active_category = props.active_category;
    let active = *active_category.read();
    let mut picker_open = use_signal::<bool>(|| false);
    let is_open = picker_open();
    let trigger_label = active.to_string();
    let category_count = SystemHotkeysCategory::ALL.len();
    let handle_toggle_picker = move |_| {
        let next = !*picker_open.read();
        picker_open.set(next);
    };
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
                aria_expanded: is_open,
                onclick: handle_toggle_picker,
                span {
                    class: "max-[1099px]:[flex:1_1_auto] max-[1099px]:text-left \
                        max-[1099px]:whitespace-nowrap max-[1099px]:overflow-hidden \
                        max-[1099px]:text-ellipsis",
                    {trigger_label}
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
                "data-open": is_open,
                for (index, category) in SystemHotkeysCategory::ALL.iter().copied().enumerate() {
                    {
                        let is_active = category == active;
                        let has_separator = index + 1 < category_count;
                        rsx! {
                            SystemHotkeysCategoryTab {
                                category,
                                is_active,
                                has_separator,
                                active_category,
                                picker_open,
                            }
                        }
                    }
                }
            }
        }
    }
}
