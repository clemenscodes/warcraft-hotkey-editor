pub mod components;
pub mod data;
mod logic;
mod props;
mod style;

use components::help_dialog_body::HelpDialogBody;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeader;
use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};
use logic::HelpDialogShell;
pub use props::HelpDialogProps;
use style::{CLASS, OVERLAY};
use tw_macro::assert_component;

assert_component!(HelpDialog);

/// The onboarding guide. It owns its own dialog shell: the shell struct shapes
/// the header and scroll body directly from props, and this places them inside
/// the backdrop and bordered box.
#[component]
pub fn HelpDialog(props: HelpDialogProps) -> Element {
    use_body_scroll_lock(props.help_open);
    let HelpDialogShell {
        open,
        on_open_change,
        header,
        body,
    } = HelpDialogShell::from(&props);
    if !open {
        return rsx! {};
    }
    rsx! {
        DialogRoot {
            class: OVERLAY,
            open,
            on_open_change,
            DialogContent {
                class: CLASS.to_library_class(),
                DialogHeader { ..header }
                HelpDialogBody { ..body }
            }
        }
    }
}
