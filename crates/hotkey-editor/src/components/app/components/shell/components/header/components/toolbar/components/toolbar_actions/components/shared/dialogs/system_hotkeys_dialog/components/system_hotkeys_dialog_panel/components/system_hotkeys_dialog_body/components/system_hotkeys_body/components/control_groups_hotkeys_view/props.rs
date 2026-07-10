use dioxus::prelude::*;
use warcraft_keybinds::WarcraftObjectId;

/// What the control-groups editor needs: the shared editing-section signal. Its
/// slots resolve their bindings from the CustomKeys query.
#[derive(Props, Clone, PartialEq)]
pub struct ControlGroupsHotkeysViewProps {
    pub editing_section: Signal<Option<WarcraftObjectId>>,
}
