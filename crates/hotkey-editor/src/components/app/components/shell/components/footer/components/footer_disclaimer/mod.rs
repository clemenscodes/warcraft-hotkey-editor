mod props;
mod style;

use dioxus::prelude::*;
use props::FooterDisclaimerProps;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn FooterDisclaimer(props: FooterDisclaimerProps) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}

assert_component!(FooterDisclaimer);
