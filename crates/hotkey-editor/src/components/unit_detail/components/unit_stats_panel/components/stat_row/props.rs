use super::state::StatRowVariant;
use dioxus::prelude::*;

/// A stat row: its colour variant, whether it is a regen sub-row, whether it is a
/// hero's primary attribute row, and the label/value/gain content.
#[derive(Props, Clone, PartialEq)]
pub struct StatRowProps {
    #[props(default)]
    pub variant: StatRowVariant,
    #[props(default)]
    pub is_regen: bool,
    #[props(default)]
    pub is_primary: bool,
    pub children: Element,
}
