use super::view::AltStatePositionButtonView;
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

impl From<&AltStatePositionButtonView> for AltStatePositionButtonProps {
    fn from(view: &AltStatePositionButtonView) -> Self {
        let AltStatePositionButtonView {
            title,
            aria_label,
            on_click,
        } = view.clone();
        Self {
            title,
            aria_label,
            on_click,
        }
    }
}

impl ddd::Props for AltStatePositionButtonProps {
    type View = AltStatePositionButtonView;
}
