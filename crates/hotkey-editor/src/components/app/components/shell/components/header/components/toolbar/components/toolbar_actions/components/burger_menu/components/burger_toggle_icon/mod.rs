mod style;

use crate::components::app::components::shell::components::shared::icons::ICON_BURGER;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
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
