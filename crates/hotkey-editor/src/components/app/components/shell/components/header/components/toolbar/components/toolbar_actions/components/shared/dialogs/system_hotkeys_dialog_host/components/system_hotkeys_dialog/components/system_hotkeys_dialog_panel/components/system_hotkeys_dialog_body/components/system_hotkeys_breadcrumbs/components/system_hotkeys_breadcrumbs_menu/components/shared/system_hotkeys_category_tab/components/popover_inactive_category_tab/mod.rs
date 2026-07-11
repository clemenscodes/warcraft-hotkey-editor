mod model;
mod view;

pub use view::PopoverInactiveCategoryTabView;
mod style;

use dioxus::prelude::*;
use model::PopoverInactiveCategoryTabModel;
use style::CLASS;
use tw_macro::assert_component;

/// An unselected category tab in the small-viewport popover: a full-width, dimmed gold
/// row, not the current page.
#[component]
pub fn PopoverInactiveCategoryTab(props: PopoverInactiveCategoryTabModel) -> Element {
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

assert_component!(PopoverInactiveCategoryTab);
