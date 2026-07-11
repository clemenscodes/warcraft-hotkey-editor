mod model;
mod view;

pub use view::FooterLinkIconView;
mod style;

use dioxus::prelude::*;
use model::FooterLinkIconModel;
use style::CLASS;
use tw_macro::assert_component;

/// The inline glyph a footer link may carry. A leaf that early-returns when the
/// link has no icon, so the parent renders it unconditionally and never branches.
#[component]
pub fn FooterLinkIcon(props: FooterLinkIconModel) -> Element {
    let Some(svg) = props.icon else {
        return rsx! {};
    };
    rsx! {
        span { class: CLASS, aria_hidden: "true", dangerous_inner_html: svg }
    }
}

assert_component!(FooterLinkIcon);
