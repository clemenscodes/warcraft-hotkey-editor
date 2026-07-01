mod logic;
mod props;
mod style;

use crate::assert_component;
use crate::components::shared::button::Button;
use dioxus::prelude::*;
use logic::DownloadInfoActionsButtons;
pub use props::DownloadInfoActionsProps;
use style::CLASS;
assert_component!(DownloadInfoActions);

/// The download dialog's right-aligned action row: the cancel and download
/// buttons.
#[component]
pub fn DownloadInfoActions(props: DownloadInfoActionsProps) -> Element {
    let DownloadInfoActionsButtons { cancel, download } = DownloadInfoActionsButtons::from(&props);
    rsx! {
        div {
            class: CLASS,
            Button { ..cancel }
            Button { ..download }
        }
    }
}
