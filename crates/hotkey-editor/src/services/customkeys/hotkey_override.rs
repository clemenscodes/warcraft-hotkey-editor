use dioxus::prelude::*;
use warcraft_keybinds::{CustomKeys, HotkeyConflict, HotkeyTarget, HotkeyToken};

use crate::model::grid::GridLayout;
use crate::model::grid::GridSlotId;

pub(crate) struct HotkeyOverride;

impl HotkeyOverride {
    pub(crate) fn apply(
        loaded_keys: &mut Signal<Option<CustomKeys>>,
        target: HotkeyTarget,
        new_token: Option<HotkeyToken>,
    ) {
        let mut writable_guard = loaded_keys.write();
        let empty_source = "";
        let file = writable_guard.get_or_insert_with(|| CustomKeys::from(empty_source));
        file.set_hotkey(target, new_token);
    }

    pub(crate) fn detect_conflict(
        container_slots: &[GridSlotId],
        target_object_id: &str,
        proposed_token: HotkeyToken,
        custom_keys: Option<&CustomKeys>,
        layout: GridLayout,
        is_research_context: bool,
    ) -> Option<HotkeyConflict> {
        let file = custom_keys?;
        file.find_hotkey_conflict(
            container_slots,
            target_object_id,
            proposed_token,
            layout,
            is_research_context,
        )
    }
}
