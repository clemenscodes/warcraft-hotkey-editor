use super::view::AboveCenterTooltipView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AboveCenterTooltipModel {
    pub text: String,
}

impl From<&AboveCenterTooltipView> for AboveCenterTooltipModel {
    fn from(view: &AboveCenterTooltipView) -> Self {
        let AboveCenterTooltipView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for AboveCenterTooltipModel {
    type View = AboveCenterTooltipView;
}
