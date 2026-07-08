use dioxus::prelude::*;
use warcraft_keybinds::WarcraftObjectId;

/// The row's inputs: the shared editing-section signal its slots share. Each slot
/// resolves its own binding from the CustomKeys query.
#[derive(Props, Clone, PartialEq)]
pub struct HeroSelectionRowProps {
    pub editing_section: Signal<Option<WarcraftObjectId>>,
}
