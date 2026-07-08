use std::fmt;

/// The capture pulse of an editable keycap: resting, or lit gold while its key
/// picker is open. The pulse look is identical wherever the keycap is used, so it
/// lives here rather than on each host.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum EditableKeycapState {
    #[default]
    Idle,
    Editing,
}

/// The corner radius an editable keycap wears — the one look axis that differs
/// between the square editor cell (`Tile`) and the layout-grid cell (`Panel`).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum EditableKeycapRadius {
    #[default]
    Tile,
    Panel,
}

impl fmt::Display for EditableKeycapRadius {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token = match self {
            Self::Tile => "tile",
            Self::Panel => "panel",
        };
        write!(formatter, "{token}")
    }
}
