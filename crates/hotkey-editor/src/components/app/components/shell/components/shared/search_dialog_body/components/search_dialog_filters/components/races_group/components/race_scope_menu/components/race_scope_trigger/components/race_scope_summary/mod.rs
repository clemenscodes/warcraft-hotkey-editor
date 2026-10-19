mod model;
mod view;

pub use view::RaceScopeSummaryView;
mod style;

use dioxus::prelude::*;
use model::RaceScopeSummaryModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn RaceScopeSummary(props: RaceScopeSummaryModel) -> Element {
    let summary = props.summary;
    rsx! {
        span {
            class: CLASS,
            {summary}
        }
    }
}

assert_component!(RaceScopeSummary);
