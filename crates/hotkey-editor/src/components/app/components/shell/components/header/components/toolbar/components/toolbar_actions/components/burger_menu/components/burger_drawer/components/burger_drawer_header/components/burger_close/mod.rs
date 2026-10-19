mod model;
mod view;

pub use view::BurgerCloseView;
mod style;

use crate::components::app::components::shell::components::shared::icons::ICON_CLOSE;
use dioxus::prelude::*;
use model::BurgerCloseModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BurgerClose(props: BurgerCloseModel) -> Element {
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_label: "Close menu",
            onclick,
            dangerous_inner_html: ICON_CLOSE,
        }
    }
}

assert_component!(BurgerClose);
