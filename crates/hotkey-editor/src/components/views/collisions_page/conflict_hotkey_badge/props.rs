use dioxus::prelude::*;
/// A conflict card's shared-hotkey badge; `is_top` places it above a multi-way row.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictHotkeyBadgeProps {
    #[props(default)]
    pub is_top: bool,
    pub children: Element,
}
