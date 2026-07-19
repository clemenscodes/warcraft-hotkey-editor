mod model;
mod view;

pub use view::InactiveResolveSectionTabView;
mod style;

use super::shared::resolve_section_tab_count::ResolveSectionTabCount;
use super::shared::resolve_section_tab_label::ResolveSectionTabLabel;
use dioxus::prelude::*;
use model::InactiveResolveSectionTabModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn InactiveResolveSectionTab(props: InactiveResolveSectionTabModel) -> Element {
    let text = props.label;
    let count = props.count;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "aria-current": "false",
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

assert_component!(InactiveResolveSectionTab);
