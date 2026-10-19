pub mod components;
mod model;
mod presentation;
mod style;
mod view;

pub use view::SectionTabBodyView;

use components::resolve_section_tab_count::ResolveSectionTabCount;
use components::resolve_section_tab_label::ResolveSectionTabLabel;
use dioxus::prelude::*;
use model::SectionTabBodyModel;
use presentation::SectionTabBodyPresentation;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SectionTabBody(props: SectionTabBodyModel) -> Element {
    let SectionTabBodyPresentation {
        text,
        count,
        current,
        onclick,
    } = SectionTabBodyPresentation::from(&props);
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "aria-current": current,
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

assert_component!(SectionTabBody);
