use dioxus::prelude::*;
use warcraft_keybinds::{CustomKeys, WarcraftObjectId};

/// The row's inputs: the loaded keys its slots edit and the shared editing-section
/// signal.
#[derive(Props, Clone, PartialEq)]
pub struct HeroSelectionRowProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub editing_section: Signal<Option<WarcraftObjectId>>,
}
