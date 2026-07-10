mod logic;
mod props;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::button::{Button, ButtonVariant};
use dioxus::prelude::*;
use logic::InfoActionsButtons;
use props::InfoActionsProps;
use style::CLASS;
use tw_macro::assert_component;

/// Every info dialog's right-aligned action row: the cancel and primary buttons.
#[component]
pub fn InfoActions(props: InfoActionsProps) -> Element {
    let InfoActionsButtons {
        cancel_label,
        on_cancel,
        primary_label,
        on_primary,
    } = InfoActionsButtons::from(&props);
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
