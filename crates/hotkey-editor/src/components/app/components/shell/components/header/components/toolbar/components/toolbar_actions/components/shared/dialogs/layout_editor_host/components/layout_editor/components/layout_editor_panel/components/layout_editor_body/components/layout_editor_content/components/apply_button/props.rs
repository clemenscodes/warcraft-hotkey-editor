use dioxus::prelude::*;

/// The apply action handler the button forwards.
#[derive(Props, Clone, PartialEq)]
pub struct ApplyButtonProps {
    pub on_apply: EventHandler<MouseEvent>,
}
