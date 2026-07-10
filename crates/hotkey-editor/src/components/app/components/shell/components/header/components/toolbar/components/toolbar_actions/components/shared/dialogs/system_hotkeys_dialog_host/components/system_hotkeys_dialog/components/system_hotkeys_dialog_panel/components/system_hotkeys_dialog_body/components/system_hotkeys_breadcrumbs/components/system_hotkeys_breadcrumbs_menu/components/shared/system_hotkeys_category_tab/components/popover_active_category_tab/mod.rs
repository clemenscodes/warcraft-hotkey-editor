mod props;
mod style;

use dioxus::prelude::*;
pub use props::PopoverActiveCategoryTabProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(PopoverActiveCategoryTab);

/// The selected category tab in the small-viewport popover: a full-width, ringed gold
/// row marked as the current page.
#[component]
pub fn PopoverActiveCategoryTab(props: PopoverActiveCategoryTabProps) -> Element {
    let label = props.label;
    let on_click = props.on_click;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            role: "option",
            aria_selected: true,
            aria_current: "page",
            onclick: on_click,
            {label}
        }
    }
}
