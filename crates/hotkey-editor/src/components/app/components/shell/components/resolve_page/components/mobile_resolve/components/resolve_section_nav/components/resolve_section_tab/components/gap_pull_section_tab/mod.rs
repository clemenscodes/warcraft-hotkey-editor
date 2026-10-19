mod model;
mod style;
mod view;

pub use view::GapPullSectionTabView;

use super::shared::section_tab_body::SectionTabBody;
use dioxus::prelude::*;
use model::GapPullSectionTabModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn GapPullSectionTab(props: GapPullSectionTabModel) -> Element {
    let label = props.label.clone();
    let count = props.count;
    let active = props.active;
    let onclick = props.onclick;
    rsx! {
        div {
            class: CLASS,
            SectionTabBody {
                label,
                count,
                active,
                onclick,
            }
        }
    }
}

assert_component!(GapPullSectionTab);
