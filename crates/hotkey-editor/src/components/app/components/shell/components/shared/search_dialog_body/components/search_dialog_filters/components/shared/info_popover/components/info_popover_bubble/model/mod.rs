use super::view::InfoPopoverBubbleView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InfoPopoverBubbleModel {
    pub text: &'static str,
    pub leaving: bool,
}

impl From<&InfoPopoverBubbleView> for InfoPopoverBubbleModel {
    fn from(view: &InfoPopoverBubbleView) -> Self {
        let InfoPopoverBubbleView { text, leaving } = view.clone();
        Self { text, leaving }
    }
}

impl ddd::Layered for InfoPopoverBubbleModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for InfoPopoverBubbleModel {
    type View = InfoPopoverBubbleView;
}
