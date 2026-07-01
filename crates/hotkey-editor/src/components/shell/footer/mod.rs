pub mod components;
mod data;
mod style;

use crate::assert_component;
use components::footer_credit::FooterCredit;
use components::footer_disclaimer::FooterDisclaimer;
use components::footer_link::FooterLink;
use components::footer_separator::FooterSeparator;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(Footer);

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: CLASS,
            FooterCredit { ..data::CREDIT }
            for link in data::LINKS.iter().cloned() {
                FooterSeparator {}
                FooterLink { ..link }
            }
            FooterDisclaimer { ..data::DISCLAIMER }
        }
    }
}
