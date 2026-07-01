mod logic;
mod props;
mod style;

use crate::assert_component;
use crate::components::shared::button::Button;
use dioxus::prelude::*;
use logic::UploadInfoActionsButtons;
pub use props::UploadInfoActionsProps;
use style::CLASS;
assert_component!(UploadInfoActions);

/// The import dialog's right-aligned action row: the cancel and choose-file
/// buttons.
#[component]
pub fn UploadInfoActions(props: UploadInfoActionsProps) -> Element {
    let UploadInfoActionsButtons {
        cancel,
        choose_file,
    } = UploadInfoActionsButtons::from(&props);
    rsx! {
        div {
            class: CLASS,
            Button { ..cancel }
            Button { ..choose_file }
        }
    }
}
