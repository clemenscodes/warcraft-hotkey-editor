mod model;
mod view;

pub use view::ActiveMobileCategoryTabView;
mod style;

use dioxus::prelude::*;
use model::ActiveMobileCategoryTabModel;
use style::CLASS;
use tw_macro::assert_component;

/// The active mobile category tab: the current category, wearing the race accent
/// (border, text, glow read from the theme's `--race-color`). Presentational.
#[component]
pub fn ActiveMobileCategoryTab(props: ActiveMobileCategoryTabModel) -> Element {
    let label = props.label;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            role: "tab",
            r#type: "button",
            aria_selected: true,
            onclick,
            {label}
        }
    }
}

assert_component!(ActiveMobileCategoryTab);
