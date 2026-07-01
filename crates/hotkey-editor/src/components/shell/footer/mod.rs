pub mod components;
mod data;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

use components::footer_credit::FooterCredit;
use components::footer_disclaimer::FooterDisclaimer;
use components::footer_link::FooterLink;
use components::footer_separator::FooterSeparator;

assert_component!(Footer);

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: CLASS,
            FooterCredit { ..data::CREDIT }
            for link in data::LINKS.iter().cloned() {
                FooterSeparator {}
                FooterLink { ..link }
            }
            FooterDisclaimer { ..data::DISCLAIMER }
        }
    }
}
