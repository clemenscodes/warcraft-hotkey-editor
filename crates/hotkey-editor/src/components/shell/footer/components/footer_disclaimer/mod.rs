mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::FooterDisclaimerProps;

assert_component!(FooterDisclaimer);

#[component]
pub fn FooterDisclaimer(props: FooterDisclaimerProps) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}
