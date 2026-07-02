use dioxus::prelude::*;

/// The colliding command-card cell shown between (or above) a conflict's abilities;
/// `is_top` stacks it over a multi-way ability row.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictPositionCellProps {
    pub collision_column: u8,
    pub collision_row: u8,
    #[props(default)]
    pub is_top: bool,
}
