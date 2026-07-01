use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

/// The export button reads the loaded file to serialize it on download; it renders
/// nothing until a file is present, since there is nothing to export otherwise.
#[derive(Props, Clone, PartialEq)]
pub struct ExportButtonProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
}
