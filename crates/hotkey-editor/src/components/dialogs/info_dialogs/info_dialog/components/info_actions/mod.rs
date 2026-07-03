mod logic;
mod props;
mod style;

use crate::assert_component;
use crate::components::dialogs::shared::button::Button;
use dioxus::prelude::*;
use logic::InfoActionsButtons;
pub use props::InfoActionsProps;
use style::CLASS;
assert_component!(InfoActions);

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
