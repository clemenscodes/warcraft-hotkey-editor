pub mod components;
mod hooks;
mod logic;
mod props;
mod style;

use components::carriers_dialog_panel::CarriersDialogPanel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogRoot;
use hooks::use_carriers_dialog;
use logic::CarriersDialogShell;
pub use props::CarriersDialogProps;
use style::CLASS;
use tw_macro::assert_component;

assert_component!(CarriersDialog);

/// Lists every unit that carries an ability in a scrollable grid; closing the dialog
/// clears the open state that summoned it. It owns its own dialog shell: the hook shapes
/// the cards and open state, the shell struct names the panel, and this places the panel
/// inside its own backdrop `div` within the library `DialogRoot`. No project class touches
/// the library element — the backdrop is this component's own classed `div`.
#[component]
pub fn CarriersDialog(props: CarriersDialogProps) -> Element {
    let view = use_carriers_dialog(&props);
    use_body_scroll_lock(view.open);
    let CarriersDialogShell {
        open,
        on_open_change,
        panel,
    } = CarriersDialogShell::from(&view);
    if !open {
        return rsx! {};
    }
    rsx! {
        DialogRoot {
            open,
            on_open_change,
            div {
                class: CLASS,
                CarriersDialogPanel { ..panel }
            }
        }
    }
}
