pub mod components;
mod model;
mod view;

pub use view::HelpDialogPanelView;
mod style;

use components::help_dialog_body::HelpDialogBody;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeader;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogContent;
use model::HelpDialogPanelModel;
use style::CLASS;
use tw_macro::assert_component;

/// The help dialog's bordered box: it wraps the library `DialogContent` (focus trap and
/// dialog semantics) and styles a real `div` of its own with the box `CLASS`, so no
/// project class ever lands on the library element. Holds the header row above the
/// scrolling body.
#[component]
pub fn HelpDialogPanel(props: HelpDialogPanelModel) -> Element {
    let title = props.title;
    let on_close = props.on_close;
    let content = props.content;
    let on_dismiss = props.on_dismiss;
    rsx! {
        DialogContent {
            div {
                class: CLASS,
                DialogHeader {
                    title,
                    on_close,
                }
                HelpDialogBody {
                    content,
                    on_dismiss,
                }
            }
        }
    }
}

assert_component!(HelpDialogPanel);
