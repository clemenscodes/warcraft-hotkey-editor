use dioxus::prelude::*;
use warcraft_keybinds::WarcraftObjectId;

/// What the hero-selection editor needs: the shared editing-section signal. Its
/// slots resolve their bindings from the CustomKeys query.
#[derive(Props, Clone, PartialEq)]
pub struct HeroSelectionHotkeysViewProps {
    pub editing_section: Signal<Option<WarcraftObjectId>>,
}
