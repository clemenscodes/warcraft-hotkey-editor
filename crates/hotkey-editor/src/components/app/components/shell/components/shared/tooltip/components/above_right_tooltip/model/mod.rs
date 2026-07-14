use super::view::AboveRightTooltipView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AboveRightTooltipModel {
    pub text: String,
}

impl From<&AboveRightTooltipView> for AboveRightTooltipModel {
    fn from(view: &AboveRightTooltipView) -> Self {
        let AboveRightTooltipView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for AboveRightTooltipModel {
    type View = AboveRightTooltipView;
}
