pub mod components;
mod hooks;
mod logic;
mod props;
mod style;

use components::carriers_dialog_body::CarriersDialogBody;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeader;
use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};
use hooks::use_carriers_dialog;
use logic::CarriersDialogShell;
pub use props::CarriersDialogProps;
use style::{CLASS, OVERLAY};
use tw_macro::assert_component;

assert_component!(CarriersDialog);

/// Lists every unit that carries an ability in a scrollable grid; closing the dialog
/// clears the open state that summoned it. It owns its own dialog shell: the hook
/// shapes the cards and open state, the shell struct names the header and scroll body.
#[component]
pub fn CarriersDialog(props: CarriersDialogProps) -> Element {
    let view = use_carriers_dialog(&props);
    use_body_scroll_lock(view.open);
    let CarriersDialogShell {
        open,
        on_open_change,
        header,
        body,
    } = CarriersDialogShell::from(&view);
    rsx! {
        DialogRoot {
            class: OVERLAY,
            open,
            on_open_change,
            DialogContent {
                class: CLASS.to_library_class(),
                DialogHeader { ..header }
                CarriersDialogBody { ..body }
            }
        }
    }
}
