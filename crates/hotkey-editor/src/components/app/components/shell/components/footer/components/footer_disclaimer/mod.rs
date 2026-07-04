mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::FooterDisclaimerProps;
use style::CLASS;
assert_component!(FooterDisclaimer);

#[component]
pub fn FooterDisclaimer(props: FooterDisclaimerProps) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}
