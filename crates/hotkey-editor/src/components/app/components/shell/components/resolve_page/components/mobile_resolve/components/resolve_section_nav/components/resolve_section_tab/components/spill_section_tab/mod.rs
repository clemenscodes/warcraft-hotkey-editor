mod model;
mod style;
mod view;

pub use view::SpillSectionTabView;

use super::shared::section_tab_body::SectionTabBody;
use dioxus::prelude::*;
use model::SpillSectionTabModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SpillSectionTab(props: SpillSectionTabModel) -> Element {
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

assert_component!(SpillSectionTab);
