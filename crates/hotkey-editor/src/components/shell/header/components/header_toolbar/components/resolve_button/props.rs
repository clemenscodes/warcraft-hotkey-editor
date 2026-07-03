use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

/// The resolve button reads the loaded file to know whether there is anything to
/// resolve; it disables itself until a file is present.
#[derive(Props, Clone, PartialEq)]
pub struct ResolveButtonProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
}
