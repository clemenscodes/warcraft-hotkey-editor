pub mod components;
mod model;
mod view;

pub use view::FooterLinkView;
mod style;

use components::footer_link_icon::FooterLinkIcon;
use dioxus::prelude::*;
use model::FooterLinkModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn FooterLink(props: FooterLinkModel) -> Element {
    let label = props.label;
    let href = props.href;
    let icon = props.icon;
    rsx! {
        a {
            class: CLASS,
            href,
            target: "_blank",
            rel: "noopener noreferrer",
            FooterLinkIcon { icon }
            {label}
        }
    }
}

assert_component!(FooterLink);
