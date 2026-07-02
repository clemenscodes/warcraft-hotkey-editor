use super::state::StatRowVariant;
use dioxus::prelude::*;

/// A stat row: the `group` whose `data-variant`/`data-regen`/`data-primary` drive
/// its children's colours. It owns only the row's shape; a semantic row component
/// (hit points, armor, damage, …) fills it with its own label and its
/// domain-typed value, formatted at the leaf.
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
