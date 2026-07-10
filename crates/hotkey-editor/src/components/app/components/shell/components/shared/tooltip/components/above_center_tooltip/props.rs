use super::view::AboveCenterTooltipView;
use dioxus::prelude::*;

/// The message this positioned bubble shows. Its placement and anchor are baked into
/// the component, so the only datum that varies is the text.
#[derive(Props, Clone, PartialEq)]
pub struct AboveCenterTooltipProps {
    pub text: String,
}

impl From<&AboveCenterTooltipView> for AboveCenterTooltipProps {
    fn from(view: &AboveCenterTooltipView) -> Self {
        let AboveCenterTooltipView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for AboveCenterTooltipProps {
    type View = AboveCenterTooltipView;
}
