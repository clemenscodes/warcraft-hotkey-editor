use dioxus::prelude::*;
use warcraft_database::SystemHotkeysCategory;

const TAB_BASE: &str = "font-friz-quadrata uppercase tracking-[0.1em] text-[2rem] leading-none \
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

#[derive(Props, Clone, PartialEq)]
pub(super) struct SystemHotkeysCategoryTabProps {
    pub(super) category: SystemHotkeysCategory,
    pub(super) is_active: bool,
    pub(super) has_separator: bool,
    pub(super) active_category: Signal<SystemHotkeysCategory>,
    pub(super) picker_open: Signal<bool>,
}

#[component]
pub(super) fn SystemHotkeysCategoryTab(props: SystemHotkeysCategoryTabProps) -> Element {
    let category = props.category;
    let is_active = props.is_active;
    let has_separator = props.has_separator;
    let mut active_category = props.active_category;
    let mut picker_open = props.picker_open;
    let tab_color = if is_active {
        "text-warcraft-gold \
        [text-shadow:1px_1px_0_#000,0_0_16px_rgba(255,206,99,0.45)] \
        group-data-[open=true]:bg-[rgba(255,206,99,0.14)] \
        group-data-[open=true]:[box-shadow:inset_0_0_0_1px_rgba(255,206,99,0.4)]"
    } else {
        "text-[rgba(255,206,99,0.55)]"
    };
    let tab_class = format!("{TAB_BASE} {tab_color}");
    let category_label = category.to_string();
    let handle_click = move |_| {
        active_category.set(category);
        picker_open.set(false);
    };
    rsx! {
        button {
            class: tab_class,
            r#type: "button",
            role: "option",
            aria_selected: is_active,
            aria_current: if is_active { "page" } else { "false" },
            onclick: handle_click,
            {category_label}
        }
        if has_separator {
            span {
                class: "font-friz-quadrata text-[2rem] leading-none select-none \
                        text-[rgba(255,206,99,0.45)] group-data-[open=true]:hidden",
                "›"
            }
        }
    }
}
