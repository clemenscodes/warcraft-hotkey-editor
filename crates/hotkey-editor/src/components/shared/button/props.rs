use dioxus::prelude::*;

/// Which weight an action button carries. Primary is the affirmative action,
/// secondary the dismissive one. The variant only selects styling.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ButtonVariant {
    Primary,
    Secondary,
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    pub variant: ButtonVariant,
    pub onclick: EventHandler<MouseEvent>,
    pub children: Element,
}
