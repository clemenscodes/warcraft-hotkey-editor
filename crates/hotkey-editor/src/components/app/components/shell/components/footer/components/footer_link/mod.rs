pub mod components;
mod props;
mod style;

use components::footer_link_icon::{FooterLinkIcon, FooterLinkIconProps};
use dioxus::prelude::*;
pub use props::FooterLinkProps;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn FooterLink(props: FooterLinkProps) -> Element {
    let icon = FooterLinkIconProps::from(&props);
    let label = props.label;
    let href = props.href;
    rsx! {
        a {
            class: CLASS,
            href,
            target: "_blank",
            rel: "noopener noreferrer",
            FooterLinkIcon { ..icon }
            {label}
        }
    }
}

assert_component!(FooterLink);
