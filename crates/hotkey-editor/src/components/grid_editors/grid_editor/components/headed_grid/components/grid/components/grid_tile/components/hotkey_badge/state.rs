/// The visual state of a hotkey badge, driving its color treatment.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum HotkeyBadgeState {
    #[default]
    Normal,
    Passive,
    Conflict,
}
