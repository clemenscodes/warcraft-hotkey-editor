mod style;

use crate::assert_component;
use crate::components::app::components::shell::components::shared::icons::ICON_BURGER;
use dioxus::prelude::*;
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
