pub mod components;
mod model;
mod view;

pub use view::CarriersDialogPanelView;
mod style;

use components::carriers_dialog_body::CarriersDialogBody;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeader;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogContent;
use model::CarriersDialogPanelModel;
use style::CLASS;
use tw_macro::assert_component;

/// The carriers dialog's bordered box: it wraps the library `DialogContent` (focus trap
/// and dialog semantics) and styles a real `div` of its own with the box `CLASS`, so no
/// project class ever lands on the library element. Holds the header row above the
/// scrolling grid of carrier cards.
#[component]
pub fn CarriersDialogPanel(props: CarriersDialogPanelModel) -> Element {
    let title = props.title;
    let on_close = props.on_close;
    let carriers = props.carriers;
    rsx! {
        DialogContent {
            div {
                class: CLASS,
                DialogHeader {
                    title,
                    on_close,
                }
                CarriersDialogBody { carriers }
            }
        }
    }
}

assert_component!(CarriersDialogPanel);
