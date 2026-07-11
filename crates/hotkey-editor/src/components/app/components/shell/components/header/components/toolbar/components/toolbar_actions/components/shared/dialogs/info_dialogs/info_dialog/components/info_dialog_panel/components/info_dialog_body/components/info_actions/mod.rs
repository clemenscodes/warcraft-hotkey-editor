mod model;
mod presentation;
mod style;
mod view;

pub use view::InfoActionsView;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::button::{Button, ButtonVariant};
use dioxus::prelude::*;
use model::InfoActionsModel;
use presentation::InfoActionsPresentation;
use style::CLASS;
use tw_macro::assert_component;

/// Every info dialog's right-aligned action row: the cancel and primary buttons.
#[component]
pub fn InfoActions(props: InfoActionsModel) -> Element {
    let InfoActionsPresentation {
        cancel_label,
        on_cancel,
        primary_label,
        on_primary,
    } = InfoActionsPresentation::from(&props);
    rsx! {
        div {
            class: CLASS,
            Button {
                variant: ButtonVariant::Secondary,
                onclick: on_cancel,
                label: cancel_label,
            }
            Button {
                variant: ButtonVariant::Primary,
                onclick: on_primary,
                label: primary_label,
            }
        }
    }
}

assert_component!(InfoActions);
