use crate::components::shared::icons::ICON_RESOLVE;
use crate::components::shell::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButtonProps;
use dioxus::prelude::*;

/// The resolve button in its two states: enabled when there is a file to resolve,
/// disabled until one is present. `onclick` routes to the Resolve page. Both are
/// supplied by `ResolveButtonHost`; the leaf itself fetches nothing.
#[derive(Props, Clone, PartialEq)]
pub struct ResolveButtonProps {
    #[props(default)]
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&ResolveButtonProps> for ToolbarButtonProps {
    fn from(props: &ResolveButtonProps) -> Self {
        let disabled = props.disabled;
        let onclick = props.onclick;
        Self {
            icon: ICON_RESOLVE,
            aria_label: "Resolve conflicts",
            disabled,
            data_action: Some("view-resolve"),
            onclick,
            ..Self::default()
        }
    }
}
