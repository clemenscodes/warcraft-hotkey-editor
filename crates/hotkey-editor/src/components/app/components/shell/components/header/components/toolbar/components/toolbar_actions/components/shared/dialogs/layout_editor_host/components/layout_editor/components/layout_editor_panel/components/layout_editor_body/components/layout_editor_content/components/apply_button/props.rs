use super::view::ApplyButtonView;
use dioxus::prelude::*;

/// The apply action handler the button forwards.
#[derive(Props, Clone, PartialEq)]
pub struct ApplyButtonProps {
    pub on_apply: EventHandler<MouseEvent>,
}

impl From<&ApplyButtonView> for ApplyButtonProps {
    fn from(view: &ApplyButtonView) -> Self {
        let ApplyButtonView { on_apply } = view.clone();
        Self { on_apply }
    }
}

impl ddd::Props for ApplyButtonProps {
    type View = ApplyButtonView;
}
