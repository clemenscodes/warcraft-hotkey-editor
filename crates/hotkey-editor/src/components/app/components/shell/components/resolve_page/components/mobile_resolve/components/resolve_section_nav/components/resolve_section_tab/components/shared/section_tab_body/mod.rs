pub mod components;
mod model;
mod view;

pub use view::SectionTabBodyView;

use components::active_resolve_section_tab::ActiveResolveSectionTab;
use components::inactive_resolve_section_tab::InactiveResolveSectionTab;
use dioxus::prelude::*;
use model::SectionTabBodyModel;
use tw_macro::assert_component;

#[component]
pub fn SectionTabBody(props: SectionTabBodyModel) -> Element {
    match props.active {
        true => {
            let label = props.label.clone();
            let count = props.count;
            let onclick = props.onclick;
            rsx! {
                ActiveResolveSectionTab {
                    label,
                    count,
                    onclick,
                }
            }
        }
        false => {
            let label = props.label.clone();
            let count = props.count;
            let onclick = props.onclick;
            rsx! {
                InactiveResolveSectionTab {
                    label,
                    count,
                    onclick,
                }
            }
        }
    }
}

assert_component!(SectionTabBody);
