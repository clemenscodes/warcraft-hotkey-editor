pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::footer_link_icon::{FooterLinkIcon, FooterLinkIconProps};
use dioxus::prelude::*;
pub use props::FooterLinkProps;
use style::CLASS;
assert_component!(FooterLink);

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
