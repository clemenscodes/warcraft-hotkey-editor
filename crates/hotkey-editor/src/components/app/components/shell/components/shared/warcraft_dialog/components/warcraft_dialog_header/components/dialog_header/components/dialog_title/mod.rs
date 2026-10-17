mod model;
mod presentation;
mod view;

pub use view::DialogTitleView;
mod style;

use dioxus::prelude::*;
use model::DialogTitleModel;
use presentation::{DialogTitlePresentation, use_dialog_title};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn DialogTitle(props: DialogTitleModel) -> Element {
    let DialogTitlePresentation { title, onmounted } = use_dialog_title(&props);
    rsx! {
        h2 {
            class: CLASS,
            onmounted: move |event| onmounted.call(event),
            {title}
        }
    }
}

assert_component!(DialogTitle);
