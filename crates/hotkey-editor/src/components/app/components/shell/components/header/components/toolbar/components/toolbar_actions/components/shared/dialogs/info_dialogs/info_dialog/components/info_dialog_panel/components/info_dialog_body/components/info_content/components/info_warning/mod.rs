mod model;
mod view;

pub use view::InfoWarningView;
mod style;

use dioxus::prelude::*;
use model::InfoWarningModel;
use style::CLASS;
use tw_macro::assert_component;

/// The amber callout warning about the fixed filename and saved positions. Only
/// the download dialog carries one; when no warning is given this renders
/// nothing.
#[component]
pub fn InfoWarning(props: InfoWarningModel) -> Element {
    let Some(warning) = props.warning else {
        return rsx! {};
    };
    rsx! {
        p { class: CLASS, "{warning}" }
    }
}

assert_component!(InfoWarning);
