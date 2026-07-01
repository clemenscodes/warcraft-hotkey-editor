use dioxus::prelude::*;

use super::props::UpgradeTierProps;

/// The tier selector's handlers: cycle the stored tier index for this object
/// backward (prev, wrapping) and forward (next, wrapping).
pub(super) struct UpgradeTierModel {
    pub(super) on_prev: EventHandler<MouseEvent>,
    pub(super) on_next: EventHandler<MouseEvent>,
}

pub(super) fn use_upgrade_tier(props: &UpgradeTierProps) -> UpgradeTierModel {
    let object_id = props.object_id;
    let total_tier_count = props.total_tier_count;
    let mut tier_overrides = props.tier_overrides;
    let on_prev = EventHandler::new(move |_event: MouseEvent| {
        let id_key = object_id.value().to_string();
        let mut writable_guard = tier_overrides.write();
        let current = writable_guard.get(id_key.as_str()).copied().unwrap_or(0);
        let next = if current == 0 {
            total_tier_count - 1
        } else {
            current - 1
        };
        writable_guard.insert(id_key, next);
    });
    let on_next = EventHandler::new(move |_event: MouseEvent| {
        let id_key = object_id.value().to_string();
        let mut writable_guard = tier_overrides.write();
        let current = writable_guard.get(id_key.as_str()).copied().unwrap_or(0);
        let next = (current + 1) % total_tier_count;
        writable_guard.insert(id_key, next);
    });
    UpgradeTierModel { on_prev, on_next }
}
