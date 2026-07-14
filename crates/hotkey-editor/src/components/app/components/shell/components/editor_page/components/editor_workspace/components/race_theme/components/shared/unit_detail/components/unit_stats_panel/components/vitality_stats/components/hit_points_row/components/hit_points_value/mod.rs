mod model;
mod view;

mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::stat_figure::StatFigure;
use dioxus::prelude::*;
use model::HitPointsValueModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HitPointsValue(props: HitPointsValueModel) -> Element {
    let value = props.value;
    let text = value.display();
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(HitPointsValue);
