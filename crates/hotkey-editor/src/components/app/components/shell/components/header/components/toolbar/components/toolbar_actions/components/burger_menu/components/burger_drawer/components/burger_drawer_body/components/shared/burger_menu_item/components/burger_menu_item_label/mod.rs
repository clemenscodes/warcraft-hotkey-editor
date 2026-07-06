mod props;
mod style;

use dioxus::prelude::*;
pub use props::BurgerMenuItemLabelProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(BurgerMenuItemLabel);

#[component]
pub fn BurgerMenuItemLabel(props: BurgerMenuItemLabelProps) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}
