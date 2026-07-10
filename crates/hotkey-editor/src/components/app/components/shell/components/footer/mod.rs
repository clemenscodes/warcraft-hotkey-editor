pub mod components;
mod data;
mod style;

use components::footer_credit::FooterCredit;
use components::footer_disclaimer::FooterDisclaimer;
use components::footer_link::FooterLink;
use components::footer_separator::FooterSeparator;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: CLASS,
            FooterCredit { ..data::CREDIT }
            for (index, link) in data::LINKS.iter().cloned().enumerate() {
                if index > 0 {
                    FooterSeparator {}
                }
                FooterLink { ..link }
            }
            FooterDisclaimer { ..data::DISCLAIMER }
        }
    }
}

assert_component!(Footer);
