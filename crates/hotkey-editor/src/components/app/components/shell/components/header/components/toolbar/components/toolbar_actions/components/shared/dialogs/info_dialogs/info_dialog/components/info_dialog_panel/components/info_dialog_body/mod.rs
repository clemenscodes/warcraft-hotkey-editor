pub mod components;
mod props;
mod style;

use components::info_actions::InfoActions;
use components::info_content::InfoContent;
use dioxus::prelude::*;
use props::InfoDialogBodyProps;
use style::CLASS;
use tw_macro::assert_component;

/// The info dialog's scrolling content region between the header and the panel
/// edge, holding the centered instruction block and the action row.
#[component]
pub fn InfoDialogBody(props: InfoDialogBodyProps) -> Element {
    let intro = props.intro;
    let warning = props.warning;
    let primary_label = props.primary_label;
    let on_primary = props.on_primary;
    let on_cancel = props.on_cancel;
    rsx! {
        div {
            class: CLASS,
            InfoContent { intro, warning }
            InfoActions { primary_label, on_primary, on_cancel }
        }
    }
}

assert_component!(InfoDialogBody);
