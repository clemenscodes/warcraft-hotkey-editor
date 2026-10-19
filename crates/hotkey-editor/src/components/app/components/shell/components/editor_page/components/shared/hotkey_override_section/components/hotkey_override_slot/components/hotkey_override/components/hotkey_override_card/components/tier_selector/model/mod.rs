use super::view::TierSelectorView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TierSelectorModel {
    pub show: bool,
    pub label: String,
    pub on_previous: EventHandler<()>,
    pub on_next: EventHandler<()>,
}

impl From<&TierSelectorView> for TierSelectorModel {
    fn from(view: &TierSelectorView) -> Self {
        let TierSelectorView {
            show,
            label,
            on_previous,
            on_next,
        } = view.clone();
        Self {
            show,
            label,
            on_previous,
            on_next,
        }
    }
}

impl ddd::Layered for TierSelectorModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for TierSelectorModel {
    type View = TierSelectorView;
}
