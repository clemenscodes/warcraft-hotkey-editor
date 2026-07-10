use super::state::{TooltipAnchor, TooltipPlacement};
use super::view::TooltipView;
use dioxus::prelude::*;

/// A hover/focus tooltip bubble shown above or below its trigger. The trigger
/// element must be a `group/tooltip relative` positioning context; this leaf
/// renders as its child and reveals on the trigger's hover or keyboard focus. An
/// empty `text` renders nothing (so a "no conflict" caption simply shows no
/// tooltip). `placement` and `anchor` default to below-center.
#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    pub text: String,
    #[props(default)]
    pub placement: TooltipPlacement,
    #[props(default)]
    pub anchor: TooltipAnchor,
}

impl From<&TooltipView> for TooltipProps {
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

impl ddd::Props for TooltipProps {
    type View = TooltipView;
}
