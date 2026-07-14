use super::state::{TooltipAnchor, TooltipPlacement};
use super::view::TooltipView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TooltipModel {
    pub text: String,
    #[props(default)]
    pub placement: TooltipPlacement,
    #[props(default)]
    pub anchor: TooltipAnchor,
}

impl From<&TooltipView> for TooltipModel {
    fn from(view: &TooltipView) -> Self {
        let TooltipView {
            text,
            placement,
            anchor,
        } = view.clone();
        Self {
            text,
            placement,
            anchor,
        }
    }
}

impl ddd::Model for TooltipModel {
    type View = TooltipView;
}
