#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum TooltipPlacement {
    #[default]
    Below,
    Above,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum TooltipAnchor {
    #[default]
    Center,
    Left,
    Right,
}
