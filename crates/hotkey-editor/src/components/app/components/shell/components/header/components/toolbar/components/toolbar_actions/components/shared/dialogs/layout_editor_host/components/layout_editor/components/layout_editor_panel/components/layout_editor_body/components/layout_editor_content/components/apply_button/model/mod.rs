use super::view::ApplyButtonView;
use dioxus::prelude::*;

/// The apply action handler the button forwards.
#[derive(Props, Clone, PartialEq)]
pub struct ApplyButtonModel {
    pub on_apply: EventHandler<MouseEvent>,
}

impl From<&ApplyButtonView> for ApplyButtonModel {
    fn from(view: &ApplyButtonView) -> Self {
        let ApplyButtonView { on_apply } = view.clone();
        Self { on_apply }
    }
}

impl ddd::Model for ApplyButtonModel {
    type View = ApplyButtonView;
}
