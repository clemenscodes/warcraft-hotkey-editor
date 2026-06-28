/// The visual state of a hotkey badge, driving its color treatment.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum HotkeyBadgeState {
    #[default]
    Normal,
    Passive,
    Conflict,
}

impl HotkeyBadgeState {
    /// The full class list for the badge in this state. The `hotkey-badge` base
    /// is always present, so the badge styles itself regardless of its parent.
    pub fn class(self) -> &'static str {
        match self {
            Self::Normal => "hotkey-badge",
            Self::Passive => "hotkey-badge passive",
            Self::Conflict => "hotkey-badge conflict",
        }
    }
}
