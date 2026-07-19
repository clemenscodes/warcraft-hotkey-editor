mod model;
mod view;

pub use view::ActiveResolveSectionTabView;
mod style;

use super::shared::resolve_section_tab_count::ResolveSectionTabCount;
use super::shared::resolve_section_tab_label::ResolveSectionTabLabel;
use dioxus::prelude::*;
use model::ActiveResolveSectionTabModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ActiveResolveSectionTab(props: ActiveResolveSectionTabModel) -> Element {
    let text = props.label;
    let count = props.count;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "aria-current": "true",
            onclick,
            ResolveSectionTabLabel {
                text,
            }
            ResolveSectionTabCount {
                count,
            }
        }
    }
}

assert_component!(ActiveResolveSectionTab);
