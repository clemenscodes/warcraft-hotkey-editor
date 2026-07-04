mod style;

use dioxus::prelude::*;

use crate::assert_component;
use crate::components::app::components::shell::components::shared::icons::ICON_SEARCH;
use style::CLASS;

assert_component!(UnitListSearchIcon);

/// The leading magnifier icon shown in the mobile search field.
#[component]
pub fn UnitListSearchIcon() -> Element {
    rsx! {
        span { class: CLASS, dangerous_inner_html: ICON_SEARCH }
    }
}
