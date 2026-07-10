mod props;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::stat_figure::StatFigure;
use dioxus::prelude::*;
pub use props::HitPointsValueProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HitPointsValue);

/// The hit-points figure: green, semibold, enlarged — the vitality headline value worn
/// directly rather than selected through a shared variant. Hit points are never muted,
/// so it renders plainly.
#[component]
pub fn HitPointsValue(props: HitPointsValueProps) -> Element {
    let value = props.value;
    let text = value.display();
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}
