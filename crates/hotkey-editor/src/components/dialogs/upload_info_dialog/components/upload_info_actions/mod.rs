mod props;
mod style;

use crate::assert_component;
use crate::components::shared::button::{Button, ButtonVariant};
use dioxus::prelude::*;
pub use props::UploadInfoActionsProps;
use style::CLASS;
assert_component!(UploadInfoActions);

/// The import dialog's right-aligned action row: the cancel and choose-file
/// buttons.
#[component]
pub fn UploadInfoActions(props: UploadInfoActionsProps) -> Element {
    let on_cancel = props.on_cancel;
    let on_choose_file = props.on_choose_file;
    rsx! {
        div { class: CLASS,
            Button { variant: ButtonVariant::Secondary, onclick: on_cancel, "Cancel" }
            Button { variant: ButtonVariant::Primary, onclick: on_choose_file, "Choose File" }
        }
    }
}
