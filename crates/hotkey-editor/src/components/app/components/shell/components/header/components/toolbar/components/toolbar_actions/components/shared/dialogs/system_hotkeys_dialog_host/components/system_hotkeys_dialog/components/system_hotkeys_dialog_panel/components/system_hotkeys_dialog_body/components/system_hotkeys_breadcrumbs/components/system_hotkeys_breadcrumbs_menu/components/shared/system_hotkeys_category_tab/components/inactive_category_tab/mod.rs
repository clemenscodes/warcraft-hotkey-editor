mod props;
mod style;

use dioxus::prelude::*;
pub use props::InactiveCategoryTabProps;
use style::CLASS;
use tw_macro::assert_component;

/// An unselected category tab: dimmed gold, not the current page.
#[component]
pub fn InactiveCategoryTab(props: InactiveCategoryTabProps) -> Element {
    let label = props.label;
    let on_click = props.on_click;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            role: "option",
            aria_selected: false,
            aria_current: "false",
            onclick: on_click,
            {label}
        }
    }
}

assert_component!(InactiveCategoryTab);
