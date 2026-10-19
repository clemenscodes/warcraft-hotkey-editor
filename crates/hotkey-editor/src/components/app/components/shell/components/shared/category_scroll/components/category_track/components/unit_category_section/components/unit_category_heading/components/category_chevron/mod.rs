mod model;
mod presentation;
mod view;

pub use view::CategoryChevronView;
mod style;

use dioxus::prelude::*;
use model::CategoryChevronModel;
use presentation::CategoryChevronPresentation;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CategoryChevron(props: CategoryChevronModel) -> Element {
    let CategoryChevronPresentation { icon } = CategoryChevronPresentation::from(&props);
    rsx! {
        span {
            class: CLASS,
            dangerous_inner_html: icon,
        }
    }
}

assert_component!(CategoryChevron);
