pub mod components;
mod props;
mod view;

pub use view::InfoDialogPanelView;
mod style;

use components::info_dialog_body::InfoDialogBody;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeader;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogContent;
use props::InfoDialogPanelProps;
use style::CLASS;
use tw_macro::assert_component;

/// The info dialog's bordered box: it wraps the library `DialogContent` (focus trap and
/// dialog semantics) and styles a real `div` of its own with the box `CLASS`, so no
/// project class ever lands on the library element. Holds the header row above the
/// scrolling body.
#[component]
pub fn InfoDialogPanel(props: InfoDialogPanelProps) -> Element {
    let title = props.title;
    let on_close = props.on_close;
    let intro = props.intro;
    let warning = props.warning;
    let primary_label = props.primary_label;
    let on_primary = props.on_primary;
    let on_cancel = props.on_cancel;
    rsx! {
        DialogContent {
            div {
                class: CLASS,
                DialogHeader { title, on_close }
                InfoDialogBody {
                    intro,
                    warning,
                    primary_label,
                    on_primary,
                    on_cancel,
                }
            }
        }
    }
}

assert_component!(InfoDialogPanel);
