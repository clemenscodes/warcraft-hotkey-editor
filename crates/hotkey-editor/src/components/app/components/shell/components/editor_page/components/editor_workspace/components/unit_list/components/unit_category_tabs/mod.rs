mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::UnitCategoryTabsProps;

assert_component!(UnitCategoryTabs);

/// The mobile category tab row in the unit picker.
#[component]
pub fn UnitCategoryTabs(props: UnitCategoryTabsProps) -> Element {
    let children = props.children;
    rsx! {
        nav {
            class: CLASS,
            role: "tablist",
            aria_label: "Unit categories",
            {children}
        }
    }
}
