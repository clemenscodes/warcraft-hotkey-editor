mod props;
mod style;

use dioxus::prelude::*;
pub use props::FooterDisclaimerProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(FooterDisclaimer);

#[component]
pub fn FooterDisclaimer(props: FooterDisclaimerProps) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}
