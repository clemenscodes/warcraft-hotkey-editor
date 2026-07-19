pub mod components;
mod model;
mod view;

pub use view::AttributesStatsHostView;
mod style;

use components::attributes_stats::AttributesStats;
use dioxus::prelude::*;
use model::AttributesStatsHostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn AttributesStatsHost(props: AttributesStatsHostModel) -> Element {
    let hero = props.hero;
    rsx! {
        div {
            class: CLASS,
            AttributesStats {
                hero,
            }
        }
    }
}

assert_component!(AttributesStatsHost);
