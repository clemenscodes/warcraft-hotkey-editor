use super::view::InfoPopoverBubbleView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InfoPopoverBubbleModel {
    pub text: &'static str,
}

impl From<&InfoPopoverBubbleView> for InfoPopoverBubbleModel {
    fn from(view: &InfoPopoverBubbleView) -> Self {
        let InfoPopoverBubbleView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for InfoPopoverBubbleModel {
    type View = InfoPopoverBubbleView;
}
