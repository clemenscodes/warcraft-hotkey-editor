mod style;

use dioxus::prelude::*;

use crate::assert_component;
use crate::components::shared::icons::ICON_BURGER;
use style::CLASS;

assert_component!(BurgerToggleIcon);

#[component]
pub fn BurgerToggleIcon() -> Element {
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
            dangerous_inner_html: ICON_BURGER,
        }
    }
}
