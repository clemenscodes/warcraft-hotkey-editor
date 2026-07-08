pub mod components;
pub mod data;
mod logic;
mod props;

use super::dialog::{Dialog, DialogProps};
use dioxus::prelude::*;
pub use props::HelpDialogProps;

/// The onboarding guide. Just a component that composes the `Dialog` base: it
/// sources the guide content and hands the body its data followed by a dismiss
/// button. It renders no element of its own.
use tw_macro::assert_component;
assert_component!(HelpDialog);
#[component]
pub fn HelpDialog(props: HelpDialogProps) -> Element {
    let help_open = props.help_open;
    if !help_open() {
        return rsx! {};
    }
    rsx! {
        Dialog { ..DialogProps::from(&props) }
    }
}
