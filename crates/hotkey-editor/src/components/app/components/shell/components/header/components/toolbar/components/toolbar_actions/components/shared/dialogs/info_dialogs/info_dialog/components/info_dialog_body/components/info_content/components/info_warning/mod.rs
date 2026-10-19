mod model;
mod view;

pub use view::InfoWarningView;
mod style;

use dioxus::prelude::*;
use model::InfoWarningModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn InfoWarning(props: InfoWarningModel) -> Element {
    let Some(warning) = props.warning else {
        return rsx! {};
    };
    rsx! {
        p {
            class: CLASS,
            "{warning}"
        }
    }
}

assert_component!(InfoWarning);
