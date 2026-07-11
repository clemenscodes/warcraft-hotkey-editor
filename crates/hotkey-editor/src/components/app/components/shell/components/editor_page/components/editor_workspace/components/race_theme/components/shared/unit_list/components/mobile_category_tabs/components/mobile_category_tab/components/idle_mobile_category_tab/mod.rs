mod model;
mod view;

pub use view::IdleMobileCategoryTabView;
mod style;

use dioxus::prelude::*;
use model::IdleMobileCategoryTabModel;
use style::CLASS;
use tw_macro::assert_component;

/// The idle mobile category tab: a muted tab that takes the race accent border on hover
/// (read from the theme's `--race-color`). Presentational.
#[component]
pub fn IdleMobileCategoryTab(props: IdleMobileCategoryTabModel) -> Element {
    let label = props.label;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            role: "tab",
            r#type: "button",
            aria_selected: false,
            onclick,
            {label}
        }
    }
}

assert_component!(IdleMobileCategoryTab);
