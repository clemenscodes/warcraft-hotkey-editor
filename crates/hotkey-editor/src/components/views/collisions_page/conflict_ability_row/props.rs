use dioxus::prelude::*;
/// A conflict card's ability row; multi-way clashes wrap instead of using the
/// two-sided grid.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictAbilityRowProps {
    #[props(default)]
    pub is_multi: bool,
    pub children: Element,
}
