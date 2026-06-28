use dioxus::prelude::*;

const TOOLBAR_BUTTON_STYLES: Asset =
    asset!("/src/components/shared/toolbar_button/toolbar_button.css");

#[derive(Props, Clone, PartialEq)]
pub struct ToolbarButtonProps {
    pub icon: &'static str,
    pub onclick: EventHandler<MouseEvent>,
    #[props(default)]
    pub disabled: bool,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// The single source of truth for how a toolbar action button looks. Consumers
/// swap only the icon, the click handler, and aria/disabled state. Styling lives
/// in `toolbar_button.css` under the `.toolbar-button` class.
#[component]
pub fn ToolbarButton(props: ToolbarButtonProps) -> Element {
    let icon = props.icon;
    let onclick = props.onclick;
    let disabled = props.disabled;
    let attributes = props.attributes;
    rsx! {
        document::Stylesheet { href: TOOLBAR_BUTTON_STYLES }
        button {
            class: "toolbar-button",
            r#type: "button",
            disabled,
            onclick,
            ..attributes,
            span { class: "toolbar-button-icon", aria_hidden: "true", dangerous_inner_html: icon }
        }
    }
}
