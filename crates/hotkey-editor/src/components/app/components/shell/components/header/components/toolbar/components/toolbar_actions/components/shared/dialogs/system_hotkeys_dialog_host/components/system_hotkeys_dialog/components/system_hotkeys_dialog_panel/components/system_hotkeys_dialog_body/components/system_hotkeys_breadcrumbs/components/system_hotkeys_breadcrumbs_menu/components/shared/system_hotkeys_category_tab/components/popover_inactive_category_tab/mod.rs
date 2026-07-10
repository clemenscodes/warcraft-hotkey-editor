mod props;
mod style;

use dioxus::prelude::*;
pub use props::PopoverInactiveCategoryTabProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(PopoverInactiveCategoryTab);

/// An unselected category tab in the small-viewport popover: a full-width, dimmed gold
/// row, not the current page.
#[component]
pub fn PopoverInactiveCategoryTab(props: PopoverInactiveCategoryTabProps) -> Element {
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
