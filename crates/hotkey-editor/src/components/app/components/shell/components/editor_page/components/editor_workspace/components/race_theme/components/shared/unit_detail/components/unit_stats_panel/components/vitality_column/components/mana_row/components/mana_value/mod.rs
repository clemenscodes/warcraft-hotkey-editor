pub mod components;
mod logic;
mod props;

use components::active_mana_value::{ActiveManaValue, ActiveManaValueProps};
use components::muted_mana_value::{MutedManaValue, MutedManaValueProps};
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::stat_figure::StatFigure;
use dioxus::prelude::*;
pub use props::ManaValueProps;
use tw_macro::assert_component;
assert_component!(ManaValue);

/// The mana figure: the human-blue accent, semibold and enlarged when the unit has a
/// mana pool, faint when it has none. A thin dispatcher — it renders the active look
/// (`ActiveManaValue`) xor the muted look (`MutedManaValue`), each built by `From`.
#[component]
pub fn ManaValue(props: ManaValueProps) -> Element {
    let is_muted = props.value.is_muted();
    if is_muted {
        let muted = MutedManaValueProps::from(&props);
        rsx! {
            MutedManaValue { ..muted }
        }
    } else {
        let active = ActiveManaValueProps::from(&props);
        rsx! {
            ActiveManaValue { ..active }
        }
    }
}
