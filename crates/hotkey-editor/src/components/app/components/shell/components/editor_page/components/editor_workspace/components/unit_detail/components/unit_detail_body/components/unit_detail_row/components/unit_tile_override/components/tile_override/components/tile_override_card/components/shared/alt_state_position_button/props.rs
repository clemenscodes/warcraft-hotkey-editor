use dioxus::prelude::*;

/// The position-picker crosshair button: its tooltip, accessible label, and the
/// click handler that opens the picker.
#[derive(Props, Clone, PartialEq)]
pub struct AltStatePositionButtonProps {
    #[props(into)]
    pub title: String,
    pub aria_label: &'static str,
    pub on_click: EventHandler<()>,
}
