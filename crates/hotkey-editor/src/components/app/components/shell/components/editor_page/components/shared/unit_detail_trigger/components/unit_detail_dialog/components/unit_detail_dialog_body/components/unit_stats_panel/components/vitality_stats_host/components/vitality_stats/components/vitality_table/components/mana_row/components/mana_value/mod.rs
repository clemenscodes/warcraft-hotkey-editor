pub mod components;
mod model;
mod view;

use components::active_mana_value::ActiveManaValue;
use components::muted_mana_value::MutedManaValue;
use crate::components::app::components::shell::components::editor_page::components::shared::unit_detail_trigger::components::unit_detail_dialog::components::unit_detail_dialog_body::components::unit_stats_panel::components::shared::stat_figure::StatFigure;
use dioxus::prelude::*;
use tw_macro::assert_component;
use model::ManaValueModel;

#[component]
pub fn ManaValue(props: ManaValueModel) -> Element {
    let value = props.value;
    let is_muted = value.is_muted();
    let text = value.display();
    if is_muted {
        rsx! {
            MutedManaValue {
                text,
            }
        }
    } else {
        rsx! {
            ActiveManaValue {
                text,
            }
        }
    }
}

assert_component!(ManaValue);
