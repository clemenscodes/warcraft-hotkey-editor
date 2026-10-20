pub mod components;
mod data;
mod presentation;
mod style;

use components::footer_credit::FooterCredit;
use components::footer_disclaimer::FooterDisclaimer;
use components::footer_link::FooterLink;
use components::footer_scroll_tuck::FooterScrollTuck;
use components::footer_separator::FooterSeparator;
use components::supported_warcraft_version::SupportedWarcraftVersion;
use dioxus::prelude::*;
use presentation::use_footer_scrolled_away;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn Footer() -> Element {
    // On mobile the footer tucks away for good once the cards are scrolled down;
    // scrolling back up leaves it tucked. Mounting FooterScrollTuck flips the
    // `has-[.footer-scroll-tuck]` collapse in this component's own style, so the footer element stays mounted and
    // its opacity transition runs. Desktop never mounts it.
    let scrolled_away = use_footer_scrolled_away();
    let credit = data::CREDIT;
    let lead = credit.lead;
    let tail = credit.tail;
    let heart = credit.heart;
    let supported_warcraft_version = data::SUPPORTED_WARCRAFT_VERSION;
    let version_label = supported_warcraft_version.label;
    let version = supported_warcraft_version.version;
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
            FooterSeparator {}
            SupportedWarcraftVersion {
                label: version_label,
                version,
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
