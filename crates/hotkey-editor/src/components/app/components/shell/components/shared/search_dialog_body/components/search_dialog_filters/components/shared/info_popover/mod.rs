pub mod components;
mod model;
mod presentation;
mod view;

pub use view::InfoPopoverView;
mod style;

use crate::components::app::components::shell::components::shared::search_dialog_body::components::search_dialog_filters::components::shared::popover_backdrop::PopoverBackdrop;
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
        is_mounted,
        is_leaving,
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
            if is_mounted {
                InfoPopoverBubble {
                    text,
                    leaving: is_leaving,
                }
            }
            if is_open {
                PopoverBackdrop {
                    onclick: dismiss,
                }
            }
        }
    }
}

assert_component!(InfoPopover);
