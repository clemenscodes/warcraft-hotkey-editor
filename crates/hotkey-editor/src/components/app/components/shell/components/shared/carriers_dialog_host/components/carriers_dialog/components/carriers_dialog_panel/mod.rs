pub mod components;
mod props;
mod style;

use components::carriers_dialog_body::{CarriersDialogBody, CarriersDialogBodyProps};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::{DialogHeader, DialogHeaderProps};
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogContent;
pub use props::CarriersDialogPanelProps;
use style::CLASS;
use tw_macro::assert_component;

/// The carriers dialog's bordered box: it wraps the library `DialogContent` (focus trap
/// and dialog semantics) and styles a real `div` of its own with the box `CLASS`, so no
/// project class ever lands on the library element. Holds the header row above the
/// scrolling grid of carrier cards.
#[component]
pub fn CarriersDialogPanel(props: CarriersDialogPanelProps) -> Element {
    let header = DialogHeaderProps::from(&props);
    let body = CarriersDialogBodyProps::from(&props);
    rsx! {
        DialogContent {
            div {
                class: CLASS,
                DialogHeader { ..header }
                CarriersDialogBody { ..body }
            }
        }
    }
}

assert_component!(CarriersDialogPanel);
