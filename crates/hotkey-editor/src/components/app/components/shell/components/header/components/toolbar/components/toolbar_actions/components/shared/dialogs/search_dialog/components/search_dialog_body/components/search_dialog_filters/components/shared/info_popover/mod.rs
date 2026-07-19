pub mod components;
mod model;
mod presentation;
mod view;

pub use view::InfoPopoverView;
mod style;

use components::info_popover_backdrop::InfoPopoverBackdrop;
use components::info_popover_bubble::InfoPopoverBubble;
use dioxus::prelude::*;
use model::InfoPopoverModel;
use presentation::{InfoPopoverPresentation, use_info_popover};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn InfoPopover(props: InfoPopoverModel) -> Element {
    let InfoPopoverPresentation {
        text,
        is_open,
        toggle,
        dismiss,
    } = use_info_popover(&props);
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_label: "Explain",
            aria_expanded: is_open,
            onclick: move |event| toggle.call(event),
            "?"
            if is_open {
                InfoPopoverBubble {
                    text,
                }
                InfoPopoverBackdrop {
                    onclick: dismiss,
                }
            }
        }
    }
}

assert_component!(InfoPopover);
