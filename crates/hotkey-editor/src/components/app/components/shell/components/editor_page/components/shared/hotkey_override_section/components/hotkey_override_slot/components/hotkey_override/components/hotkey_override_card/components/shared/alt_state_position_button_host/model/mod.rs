use super::view::AltStatePositionButtonHostView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AltStatePositionButtonHostModel {
    #[props(into)]
    pub title: String,
    pub aria_label: &'static str,
    pub on_click: EventHandler<()>,
}

impl From<&AltStatePositionButtonHostView> for AltStatePositionButtonHostModel {
    fn from(view: &AltStatePositionButtonHostView) -> Self {
        let AltStatePositionButtonHostView {
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

impl ddd::Model for AltStatePositionButtonHostModel {
    type View = AltStatePositionButtonHostView;
}
