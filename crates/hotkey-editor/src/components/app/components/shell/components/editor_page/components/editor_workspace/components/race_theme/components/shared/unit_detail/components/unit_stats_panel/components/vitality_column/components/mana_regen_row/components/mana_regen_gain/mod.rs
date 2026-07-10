mod props;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::stat_figure::StatFigure;
use dioxus::prelude::*;
pub use props::ManaRegenGainProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ManaRegenGain);

/// The mana-regeneration gain: the human-blue accent, pushed to the row's end, dimmed
/// when the unit does not regenerate mana.
#[component]
pub fn ManaRegenGain(props: ManaRegenGainProps) -> Element {
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
