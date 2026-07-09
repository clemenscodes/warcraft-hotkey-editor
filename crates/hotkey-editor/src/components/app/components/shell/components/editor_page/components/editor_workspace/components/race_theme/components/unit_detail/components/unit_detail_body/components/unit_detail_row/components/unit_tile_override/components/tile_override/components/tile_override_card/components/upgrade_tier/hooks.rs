use dioxus::prelude::*;

use super::components::tile_override_tier_button::TileOverrideTierButtonProps;
use super::components::tile_override_tier_label::TileOverrideTierLabelProps;
use super::props::UpgradeTierProps;
use crate::components::app::components::shell::components::shared::icons::{
    ICON_TIER_NEXT, ICON_TIER_PREV,
};

/// The tier selector's finished children: the prev/next arrow buttons (each with
/// its wrapping cycle handler) around the caption.
pub(super) struct UpgradeTierModel {
    pub(super) prev_button: TileOverrideTierButtonProps,
    pub(super) label: TileOverrideTierLabelProps,
    pub(super) next_button: TileOverrideTierButtonProps,
}

pub(super) fn use_upgrade_tier(props: &UpgradeTierProps) -> UpgradeTierModel {
    let object_id = props.object_id;
    let total_tier_count = props.total_tier_count;
    let mut tier_overrides = props.tier_overrides;
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
    let prev_button = TileOverrideTierButtonProps {
        aria_label: "Previous level",
        icon: ICON_TIER_PREV,
        on_click: on_prev,
    };
    let label = TileOverrideTierLabelProps {
        text: props.tier_label_text.clone(),
    };
    let next_button = TileOverrideTierButtonProps {
        aria_label: "Next level",
        icon: ICON_TIER_NEXT,
        on_click: on_next,
    };
    UpgradeTierModel {
        prev_button,
        label,
        next_button,
    }
}
