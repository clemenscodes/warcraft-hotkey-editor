mod model;
mod view;

pub use view::BurgerMenuItemLabelView;
mod style;

use dioxus::prelude::*;
use model::BurgerMenuItemLabelModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BurgerMenuItemLabel(props: BurgerMenuItemLabelModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(BurgerMenuItemLabel);
