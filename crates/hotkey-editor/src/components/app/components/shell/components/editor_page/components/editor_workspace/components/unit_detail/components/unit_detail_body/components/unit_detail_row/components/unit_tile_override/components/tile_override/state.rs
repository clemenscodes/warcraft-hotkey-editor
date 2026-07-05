/// Which field the hotkey picker is currently editing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum OverrideEditTarget {
    Hotkey,
    ResearchHotkey,
    /// Off-state hotkey of a toggle ability — Stop Defend, Unburrow, unmorph. Writes
    /// the `Unhotkey` field rather than `Hotkey`.
    AltHotkey,
    /// Hotkey for the upgraded-form unit that shares this button position (e.g.
    /// post-Barrage Siege Engine). Writes to the upgrade unit's own `Hotkey=` binding.
    UpgradeHotkey,
}
