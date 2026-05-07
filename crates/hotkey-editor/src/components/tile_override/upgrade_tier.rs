use std::collections::HashMap;

use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// Tier-cycling footer for multi-level abilities (upgrades that have distinct
/// ubertip/name text per level). Shows a prev/next button and a "Level N of M"
/// label.
#[component]
pub(crate) fn UpgradeTierSelector(
    object_id: WarcraftObjectId,
    active_tier_index: usize,
    total_tier_count: usize,
    tier_label_text: String,
    mut tier_overrides: Signal<HashMap<String, usize>>,
) -> Element {
    let prev_object_id = object_id;
    let next_object_id = object_id;
    rsx! {
        div { class: "tile-override-tier-footer",
            button {
                class: "tile-override-tier-button",
                aria_label: "Previous level",
                onclick: move |_| {
                    let tier_count = total_tier_count;
                    let id_key = prev_object_id.value().to_string();
                    let mut writable_guard = tier_overrides.write();
                    let current = writable_guard.get(id_key.as_str()).copied().unwrap_or(0);
                    let next = if current == 0 { tier_count - 1 } else { current - 1 };
                    writable_guard.insert(id_key, next);
                }
            }
            span { class: "tile-override-tier-label", "{tier_label_text}" }
            button {
                class: "tile-override-tier-button",
                aria_label: "Next level",
                onclick: move |_| {
                    let tier_count = total_tier_count;
                    let id_key = next_object_id.value().to_string();
                    let mut writable_guard = tier_overrides.write();
                    let current = writable_guard.get(id_key.as_str()).copied().unwrap_or(0);
                    let next = (current + 1) % tier_count;
                    writable_guard.insert(id_key, next);
                }
            }
        }
    }
}
