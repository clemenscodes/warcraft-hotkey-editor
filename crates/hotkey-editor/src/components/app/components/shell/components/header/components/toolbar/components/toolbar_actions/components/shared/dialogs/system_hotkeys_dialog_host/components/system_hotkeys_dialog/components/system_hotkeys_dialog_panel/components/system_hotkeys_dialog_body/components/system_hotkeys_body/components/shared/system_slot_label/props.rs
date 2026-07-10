use super::view::SystemSlotLabelView;
use dioxus::prelude::*;

/// A slot's caption (e.g. "SLOT 1", "HERO 2"). The tighter control-group density is
/// owned by the parent size container, so the caption carries no density flag.
#[derive(Props, Clone, PartialEq)]
pub struct SystemSlotLabelProps {
    #[props(into)]
    pub text: String,
}

impl From<&SystemSlotLabelView> for SystemSlotLabelProps {
    fn from(view: &SystemSlotLabelView) -> Self {
        let SystemSlotLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for SystemSlotLabelProps {
    type View = SystemSlotLabelView;
}
