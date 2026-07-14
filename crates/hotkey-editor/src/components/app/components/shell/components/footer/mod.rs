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
    let credit = data::CREDIT;
    let lead = credit.lead;
    let tail = credit.tail;
    let heart = credit.heart;
    rsx! {
        footer {
            class: CLASS,
            FooterCredit {
                lead,
                tail,
                heart,
            }
            for (index, link) in data::LINKS.iter().enumerate() {
                if index > 0 {
                    FooterSeparator {}
                }
                FooterLink {
                    label: link.label,
                    href: link.href,
                    icon: link.icon,
                }
            }
            FooterDisclaimer {
                text: data::DISCLAIMER,
            }
        }
    }
}

assert_component!(Footer);
