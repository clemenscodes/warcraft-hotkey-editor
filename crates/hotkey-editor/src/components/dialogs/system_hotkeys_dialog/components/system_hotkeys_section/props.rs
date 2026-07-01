use dioxus::prelude::*;

/// A category editor's frame: the one-line intro caption above it, and the editor
/// content (a slot grid, row, or list) as children.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysSectionProps {
    #[props(into)]
    pub intro: String,
    pub children: Element,
}
