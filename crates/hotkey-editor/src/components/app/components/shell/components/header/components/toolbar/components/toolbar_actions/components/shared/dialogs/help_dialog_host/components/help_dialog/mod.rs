pub mod components;
pub mod data;
mod logic;
mod props;
mod style;

use components::help_dialog_panel::HelpDialogPanel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogRoot;
use logic::HelpDialogShell;
use props::HelpDialogProps;
use style::CLASS;
use tw_macro::assert_component;

/// The onboarding guide. It owns its own dialog shell: the shell struct shapes the
/// panel from props, and this places the panel inside its own backdrop `div` (the
/// dimmed, centring layer) within the library `DialogRoot`. No project class touches
/// the library element — the backdrop is this component's own classed `div`.
#[component]
pub fn HelpDialog(props: HelpDialogProps) -> Element {
    use_body_scroll_lock(props.help_open);
    let HelpDialogShell {
        open,
        on_open_change,
        title,
        on_close,
        content,
        on_dismiss,
    } = HelpDialogShell::from(&props);
    if !open {
        return rsx! {};
    }
    rsx! {
        DialogRoot {
            open,
            on_open_change,
            div {
                class: CLASS,
                HelpDialogPanel {
                    title,
                    on_close,
                    content,
                    on_dismiss,
                }
            }
        }
    }
}

assert_component!(HelpDialog);
