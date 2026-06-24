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
        div { class: "tile-override-tier-footer",
            button {
                class: "tile-override-tier-button",
                aria_label: "Previous level",
                onclick: handle_prev,
                span {
                    class: "tile-override-tier-icon",
                    aria_hidden: "true",
                    dangerous_inner_html: ICON_TIER_PREV,
                }
            }
            span {
                class: "tile-override-tier-label",
                {tier_label_text}
            }
            button {
                class: "tile-override-tier-button",
                aria_label: "Next level",
                onclick: handle_next,
                span {
                    class: "tile-override-tier-icon",
                    aria_hidden: "true",
                    dangerous_inner_html: ICON_TIER_NEXT,
                }
            }
        }
    }
}
