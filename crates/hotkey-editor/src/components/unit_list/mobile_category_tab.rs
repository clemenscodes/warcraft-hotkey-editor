use dioxus::prelude::*;
use warcraft_api::UnitKind;
use warcraft_database::UnitKindHelpers;

use super::unit_kind_data_attr;

#[component]
pub(super) fn MobileCategoryTab(
    kind: UnitKind,
    is_active: bool,
    mut active_category: Signal<UnitKind>,
) -> Element {
    let label = UnitKindHelpers::category_label(kind);
    let kind_attr = unit_kind_data_attr(kind);
    let handle_click = move |_| active_category.set(kind);
    rsx! {
        button {
            class: "flex-1 min-w-0 min-h-[44px] px-2 \
                    bg-[rgba(13,31,61,0.55)] border border-[#1f3d63] rounded-[8px] \
                    text-[#c0c8d4] font-friz-quadrata text-[0.95rem] tracking-[0.04em] uppercase \
                    text-center cursor-pointer transition-all duration-[120ms] \
                    whitespace-nowrap overflow-hidden text-ellipsis \
                    hover:bg-[rgba(30,60,95,0.7)] hover:text-white hover:border-warcraft-blue \
                    focus:outline-none \
                    [body[data-kb-modality]_&]:focus:outline-none \
                    [body[data-kb-modality]_&]:focus:border-white \
                    [body[data-kb-modality]_&]:focus:shadow-[0_0_0_2px_#fff] \
                    data-[active=true]:bg-gradient-to-br \
                    data-[active=true]:from-[rgba(45,80,130,0.95)] \
                    data-[active=true]:to-[rgba(20,45,80,0.95)] \
                    data-[active=true]:border-warcraft-gold \
                    data-[active=true]:text-warcraft-gold \
                    data-[active=true]:shadow-[0_0_6px_rgba(255,206,99,0.3)]",
            role: "tab",
            r#type: "button",
            aria_selected: is_active,
            "data-unit-kind": kind_attr,
            "data-active": is_active,
            onclick: handle_click,
            "{label}"
        }
    }
}
