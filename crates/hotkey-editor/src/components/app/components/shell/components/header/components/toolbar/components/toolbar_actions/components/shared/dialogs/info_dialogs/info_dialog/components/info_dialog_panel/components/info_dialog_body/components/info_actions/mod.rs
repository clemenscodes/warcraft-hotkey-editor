mod logic;
mod props;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::button::Button;
use dioxus::prelude::*;
use logic::InfoActionsButtons;
pub use props::InfoActionsProps;
use style::CLASS;
use tw_macro::assert_component;

/// Every info dialog's right-aligned action row: the cancel and primary buttons.
#[component]
pub fn InfoActions(props: InfoActionsProps) -> Element {
    let InfoActionsButtons { cancel, primary } = InfoActionsButtons::from(&props);
    rsx! {
        div {
            class: CLASS,
            Button { ..cancel }
            Button { ..primary }
        }
    }
}

assert_component!(InfoActions);
