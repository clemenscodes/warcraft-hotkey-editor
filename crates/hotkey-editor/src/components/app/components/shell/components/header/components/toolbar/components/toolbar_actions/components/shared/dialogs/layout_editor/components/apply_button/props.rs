use dioxus::prelude::*;

/// The apply action handler the footer button forwards.
#[derive(Props, Clone, PartialEq)]
pub struct ApplyButtonProps {
    pub on_apply: EventHandler<MouseEvent>,
}
