pub mod components;
mod props;
mod view;

pub use view::TemplatesDialogPanelView;
mod style;

use components::templates_dialog_body::TemplatesDialogBody;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeader;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogContent;
use props::TemplatesDialogPanelProps;
use style::CLASS;
use tw_macro::assert_component;

/// The templates dialog's bordered box: it wraps the library `DialogContent` (focus trap
/// and dialog semantics) and styles a real `div` of its own with the box `CLASS`, so no
/// project class ever lands on the library element. Holds the header row above the
/// scrolling body.
#[component]
pub fn TemplatesDialogPanel(props: TemplatesDialogPanelProps) -> Element {
    let title = props.title;
    let on_close = props.on_close;
    let cards = props.cards;
    rsx! {
        DialogContent {
            div {
                class: CLASS,
                DialogHeader { title, on_close }
                TemplatesDialogBody { cards }
            }
        }
    }
}

assert_component!(TemplatesDialogPanel);
