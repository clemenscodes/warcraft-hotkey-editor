mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::InfoWarningProps;
use style::CLASS;
assert_component!(InfoWarning);

/// The amber callout warning about the fixed filename and saved positions. Only
/// the download dialog carries one; when no warning is given this renders
/// nothing.
#[component]
pub fn InfoWarning(props: InfoWarningProps) -> Element {
    let Some(warning) = props.warning else {
        return rsx! {};
    };
    rsx! {
        p { class: CLASS, "{warning}" }
    }
}
