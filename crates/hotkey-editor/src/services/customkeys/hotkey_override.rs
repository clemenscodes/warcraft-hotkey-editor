use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridLayout;
use warcraft_keybinds::GridSlotId;
use warcraft_keybinds::{CustomKeys, HotkeyConflict, HotkeyToken};

pub(crate) fn detect_conflict(
    container_slots: &[GridSlotId],
    target_object_id: WarcraftObjectId,
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
