mod model;
mod view;

pub use view::ToastTitleView;
mod style;

use dioxus::prelude::*;
use model::ToastTitleModel;
use style::CLASS;
use tw_macro::assert_component;

/// The toast headline: the uppercase heading look tinted by `--toast-title`.
#[component]
pub fn ToastTitle(props: ToastTitleModel) -> Element {
    let title = props.title;
    rsx! {
        div {
            class: CLASS,
            {title}
        }
    }
}

assert_component!(ToastTitle);
