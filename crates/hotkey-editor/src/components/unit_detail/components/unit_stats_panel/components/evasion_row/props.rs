use dioxus::prelude::*;
use warcraft_keybinds::Evasion;

/// The evasion row's input: the unit's resolved dodge chance.
#[derive(Props, Clone, PartialEq)]
pub struct EvasionRowProps {
    pub evasion: Evasion,
}
