mod model;
mod view;

mod style;

use dioxus::prelude::*;
use model::BurgerDrawerTitleModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BurgerDrawerTitle(props: BurgerDrawerTitleModel) -> Element {
    let title = props.title;
    rsx! {
        h2 {
            class: CLASS,
            {title}
        }
    }
}

assert_component!(BurgerDrawerTitle);
