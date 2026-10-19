mod model;
mod style;
mod view;

pub use view::RaceScopeBadgeView;

use crate::components::app::components::shell::components::shared::race_tab_banner::RaceTabBanner;
use dioxus::prelude::*;
use model::RaceScopeBadgeModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn RaceScopeBadge(props: RaceScopeBadgeModel) -> Element {
    let RaceScopeBadgeModel {
        race,
        is_active,
        label,
        onclick,
        onkeydown,
    } = props;
    rsx! {
        div {
            class: CLASS,
            RaceTabBanner {
                race,
                is_active,
                label,
                onclick,
                onkeydown,
            }
        }
    }
}

assert_component!(RaceScopeBadge);
