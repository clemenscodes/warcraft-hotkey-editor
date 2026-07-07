pub mod components;
mod hooks;
mod logic;
mod props;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::dialog::Dialog;
use components::carriers_grid::CarriersGrid;
use dioxus::prelude::*;
use hooks::{CarriersDialogView, use_carriers_dialog};
pub use props::CarriersDialogProps;

/// Lists every unit that carries an ability in a scrollable grid; closing the dialog
/// clears the open state that summoned it.
use tw_macro::assert_component;
assert_component!(CarriersDialog);
#[component]
pub fn CarriersDialog(props: CarriersDialogProps) -> Element {
    let CarriersDialogView { open, title, cards } = use_carriers_dialog(&props);
    rsx! {
        Dialog { open, title,
            CarriersGrid { cards }
        }
    }
}
