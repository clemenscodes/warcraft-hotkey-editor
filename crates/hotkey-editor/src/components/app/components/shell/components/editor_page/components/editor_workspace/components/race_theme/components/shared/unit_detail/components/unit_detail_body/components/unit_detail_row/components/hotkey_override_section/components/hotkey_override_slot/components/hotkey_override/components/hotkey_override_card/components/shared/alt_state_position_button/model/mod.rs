use super::view::AltStatePositionButtonView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AltStatePositionButtonModel {
    #[props(into)]
    pub title: String,
    pub aria_label: &'static str,
    pub on_click: EventHandler<()>,
}

impl From<&AltStatePositionButtonView> for AltStatePositionButtonModel {
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

impl ddd::Model for AltStatePositionButtonModel {
    type View = AltStatePositionButtonView;
}
