mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// An invisible marker the footer mounts while it should be tucked away. It renders
/// nothing the user sees; its only job is to carry the `.footer-scroll-tuck`
/// identity so the footer's own style can react with `has-[.footer-scroll-tuck]`.
#[component]
pub fn FooterScrollTuck() -> Element {
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
        }
    }
}

assert_component!(FooterScrollTuck);
