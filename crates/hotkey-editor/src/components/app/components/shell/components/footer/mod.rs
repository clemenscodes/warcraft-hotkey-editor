pub mod components;
mod data;
mod presentation;
mod style;

use components::footer_credit::FooterCredit;
use components::footer_disclaimer::FooterDisclaimer;
use components::footer_link::FooterLink;
use components::footer_scroll_tuck::FooterScrollTuck;
use components::footer_separator::FooterSeparator;
use dioxus::prelude::*;
use presentation::use_footer_scrolled_away;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn Footer() -> Element {
    // On mobile the footer tucks away while the cards are scrolled down and comes
    // back on scroll up. Mounting FooterScrollTuck flips the `has-[.footer-scroll-tuck]`
    // collapse in this component's own style, so the footer element stays mounted and
    // its height transition runs. Desktop never mounts it.
    let scrolled_away = use_footer_scrolled_away();
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
            if scrolled_away {
                FooterScrollTuck {}
            }
        }
    }
}

assert_component!(Footer);
