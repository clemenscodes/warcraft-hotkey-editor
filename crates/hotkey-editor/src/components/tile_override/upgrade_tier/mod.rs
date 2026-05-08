use std::collections::HashMap;

use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

use crate::components::shared::icons::{ICON_TIER_NEXT, ICON_TIER_PREV};

/// Tier-cycling footer for multi-level abilities (upgrades that have distinct
/// ubertip/name text per level). Shows a prev/next button and a "Level N of M"
/// label.
#[derive(Props, Clone, PartialEq)]
pub(crate) struct UpgradeTierSelectorProps {
    pub(crate) object_id: WarcraftObjectId,
    pub(crate) active_tier_index: usize,
    pub(crate) total_tier_count: usize,
    pub(crate) tier_label_text: String,
    pub(crate) tier_overrides: Signal<HashMap<String, usize>>,
}

#[component]
pub(crate) fn UpgradeTierSelector(props: UpgradeTierSelectorProps) -> Element {
    let object_id = props.object_id;
    let _active_tier_index = props.active_tier_index;
    let total_tier_count = props.total_tier_count;
    let tier_label_text = props.tier_label_text;
    let mut tier_overrides = props.tier_overrides;
    let prev_object_id = object_id;
    let next_object_id = object_id;
    let handle_prev = move |_| {
        let tier_count = total_tier_count;
        let id_key = prev_object_id.value().to_string();
        let mut writable_guard = tier_overrides.write();
        let current = writable_guard.get(id_key.as_str()).copied().unwrap_or(0);
        let next = if current == 0 {
            tier_count - 1
        } else {
            current - 1
        };
        writable_guard.insert(id_key, next);
    };
    let handle_next = move |_| {
        let tier_count = total_tier_count;
        let id_key = next_object_id.value().to_string();
        let mut writable_guard = tier_overrides.write();
        let current = writable_guard.get(id_key.as_str()).copied().unwrap_or(0);
        let next = (current + 1) % tier_count;
        writable_guard.insert(id_key, next);
    };
    rsx! {
        div { class: "mt-auto flex items-center justify-center gap-[0.85rem] pt-4",
            button {
                class: "tile-override-tier-button w-[2.4rem] h-[2.4rem] p-0 bg-[rgba(40,30,8,0.55)] border border-[#6c5a1f] rounded cursor-pointer flex items-center justify-center transition-[border-color,background] duration-[120ms] hover:border-warcraft-gold hover:bg-[rgba(255,206,99,0.12)] max-[1099px]:w-[34px] max-[1099px]:h-[34px] max-[1099px]:min-w-[34px] max-[1099px]:min-h-[34px]",
                aria_label: "Previous level",
                onclick: handle_prev,
                span {
                    class: "block w-[1.7rem] h-[1.7rem] max-[1099px]:w-[22px] max-[1099px]:h-[22px]",
                    aria_hidden: "true",
                    dangerous_inner_html: ICON_TIER_PREV,
                }
            }
            span {
                class: "font-friz-quadrata uppercase tracking-[0.06em] text-[1.3rem] text-warcraft-text-secondary",
                {tier_label_text}
            }
            button {
                class: "tile-override-tier-button w-[2.4rem] h-[2.4rem] p-0 bg-[rgba(40,30,8,0.55)] border border-[#6c5a1f] rounded cursor-pointer flex items-center justify-center transition-[border-color,background] duration-[120ms] hover:border-warcraft-gold hover:bg-[rgba(255,206,99,0.12)] max-[1099px]:w-[34px] max-[1099px]:h-[34px] max-[1099px]:min-w-[34px] max-[1099px]:min-h-[34px]",
                aria_label: "Next level",
                onclick: handle_next,
                span {
                    class: "block w-[1.7rem] h-[1.7rem] max-[1099px]:w-[22px] max-[1099px]:h-[22px]",
                    aria_hidden: "true",
                    dangerous_inner_html: ICON_TIER_NEXT,
                }
            }
        }
    }
}
