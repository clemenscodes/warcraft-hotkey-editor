mod props;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

pub use props::CatalogVisibilityButtonProps;

assert_component!(CatalogVisibilityButton);

/// One button of the catalog-visibility toggle.
#[component]
pub fn CatalogVisibilityButton(props: CatalogVisibilityButtonProps) -> Element {
    let label = props.label;
    let title = props.title;
    let is_active = props.is_active;
    let on_toggle = props.on_toggle;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            title,
            "data-active": is_active,
            aria_pressed: is_active,
            onclick: on_toggle,
            {label}
        }
    }
}
