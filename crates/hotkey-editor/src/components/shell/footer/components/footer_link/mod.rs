pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

use components::footer_link_icon::{FooterLinkIcon, FooterLinkIconProps};

pub use props::FooterLinkProps;

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
