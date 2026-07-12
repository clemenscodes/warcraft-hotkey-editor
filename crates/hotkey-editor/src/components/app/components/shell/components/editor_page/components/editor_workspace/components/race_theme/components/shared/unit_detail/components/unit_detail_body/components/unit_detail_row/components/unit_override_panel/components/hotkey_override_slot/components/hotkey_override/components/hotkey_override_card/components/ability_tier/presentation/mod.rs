use dioxus::prelude::*;

use super::model::AbilityTierModel;
use crate::services::editor_state::context::use_editor_state;

/// The tier selector's finished data: the prev/next cycle handlers around the
/// caption text.
pub(super) struct AbilityTierPresentation {
    pub(super) on_prev: EventHandler<MouseEvent>,
    pub(super) on_next: EventHandler<MouseEvent>,
    pub(super) tier_label_text: String,
}

pub(super) fn use_ability_tier(props: &AbilityTierModel) -> AbilityTierPresentation {
    let object_id = props.object_id;
    let total_tier_count = props.total_tier_count;
    let mut tier_overrides = use_editor_state().tier_overrides();
    let on_prev = EventHandler::new(move |_event: MouseEvent| {
        let mut writable_guard = tier_overrides.write();
        let current = writable_guard.get(&object_id).copied().unwrap_or(0);
        let next = if current == 0 {
            total_tier_count - 1
        } else {
            current - 1
        };
        writable_guard.insert(object_id, next);
    });
    let on_next = EventHandler::new(move |_event: MouseEvent| {
        let mut writable_guard = tier_overrides.write();
        let current = writable_guard.get(&object_id).copied().unwrap_or(0);
        let next = (current + 1) % total_tier_count;
        writable_guard.insert(object_id, next);
    });
    let tier_label_text = props.tier_label_text.clone();
    AbilityTierPresentation {
        on_prev,
        on_next,
        tier_label_text,
    }
}

impl ddd::Presentation for AbilityTierPresentation {
    type Model = AbilityTierModel;
}
