/// Where a [`Tooltip`](super::Tooltip) sits vertically relative to its trigger.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum TooltipPlacement {
    #[default]
    Below,
    Above,
}

/// How a [`Tooltip`](super::Tooltip) aligns horizontally to its trigger: centered
/// on it, or pinned to the trigger's left or right edge so an edge tooltip stays
/// on-screen.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum TooltipAnchor {
    #[default]
    Center,
    Left,
    Right,
}
