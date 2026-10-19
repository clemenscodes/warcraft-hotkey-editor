#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum HotkeyBadgeState {
    #[default]
    Normal,
    Passive,
    Conflict,
}
