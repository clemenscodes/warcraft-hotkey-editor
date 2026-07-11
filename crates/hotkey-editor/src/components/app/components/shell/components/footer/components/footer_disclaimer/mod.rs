mod model;
mod view;

pub use view::FooterDisclaimerView;
mod style;

use dioxus::prelude::*;
use model::FooterDisclaimerModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn FooterDisclaimer(props: FooterDisclaimerModel) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}

assert_component!(FooterDisclaimer);
