mod model;
mod view;

pub use view::DialogTitleView;
mod style;

use dioxus::prelude::*;
use model::DialogTitleModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn DialogTitle(props: DialogTitleModel) -> Element {
    let title = props.title;
    rsx! {
        h2 {
            class: CLASS,
            {title}
        }
    }
}

assert_component!(DialogTitle);
