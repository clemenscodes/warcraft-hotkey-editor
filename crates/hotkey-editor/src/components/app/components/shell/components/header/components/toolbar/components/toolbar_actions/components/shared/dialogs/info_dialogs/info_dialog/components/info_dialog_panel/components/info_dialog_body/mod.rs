pub mod components;
mod props;
mod style;

use components::info_actions::{InfoActions, InfoActionsProps};
use components::info_content::{InfoContent, InfoContentProps};
use dioxus::prelude::*;
pub use props::InfoDialogBodyProps;
use style::CLASS;
use tw_macro::assert_component;

/// The info dialog's scrolling content region between the header and the panel
/// edge, holding the centered instruction block and the action row.
#[component]
pub fn InfoDialogBody(props: InfoDialogBodyProps) -> Element {
    let content = InfoContentProps::from(&props);
    let actions = InfoActionsProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            InfoContent { ..content }
            InfoActions { ..actions }
        }
    }
}

assert_component!(InfoDialogBody);
