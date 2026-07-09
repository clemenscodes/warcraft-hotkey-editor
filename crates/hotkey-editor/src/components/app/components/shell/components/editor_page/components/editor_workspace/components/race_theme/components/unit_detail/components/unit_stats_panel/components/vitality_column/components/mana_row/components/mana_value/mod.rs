mod props;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_stats_panel::components::shared::stat_figure::StatFigure;
use dioxus::prelude::*;
pub use props::ManaValueProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ManaValue);

/// The mana figure: the human-blue accent, semibold and enlarged, dimmed to faint when
/// the unit has no mana pool. Worn directly rather than selected through a variant.
#[component]
pub fn ManaValue(props: ManaValueProps) -> Element {
    let value = props.value;
    let is_muted = value.is_muted();
    let text = value.display();
    rsx! {
        span {
            class: CLASS,
            "data-zero": is_muted,
            {text}
        }
    }
}
