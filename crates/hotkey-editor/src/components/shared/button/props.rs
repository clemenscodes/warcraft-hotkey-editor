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

/// The button's resolved class list and forwarded click, built from the variant
/// so the body only places them.
pub(super) struct ButtonPresentation {
    pub(super) class: String,
    pub(super) onclick: EventHandler<MouseEvent>,
}

impl From<&ButtonProps> for ButtonPresentation {
    fn from(props: &ButtonProps) -> Self {
        let modifier_class = match props.variant {
            ButtonVariant::Primary => "button-primary",
            ButtonVariant::Secondary => "button-secondary",
        };
        let class = format!("button {modifier_class}");
        let onclick = props.onclick;
        Self { class, onclick }
    }
}
