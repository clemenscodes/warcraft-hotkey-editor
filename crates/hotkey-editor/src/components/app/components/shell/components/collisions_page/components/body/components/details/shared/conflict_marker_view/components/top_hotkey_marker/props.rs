use dioxus::prelude::*;

/// The shared-hotkey badge capping a multi-way stack (nudged down).
#[derive(Props, Clone, PartialEq)]
pub struct TopHotkeyMarkerProps {
    #[props(into)]
    pub label: String,
}
