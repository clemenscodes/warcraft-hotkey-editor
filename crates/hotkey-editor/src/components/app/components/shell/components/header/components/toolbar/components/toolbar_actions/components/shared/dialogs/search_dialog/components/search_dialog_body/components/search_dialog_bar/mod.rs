pub mod components;
mod model;
mod view;

pub use view::SearchDialogBarView;
mod style;

use crate::components::app::components::shell::components::shared::unit_list_search::components::unit_list_search_icon::UnitListSearchIcon;
use crate::components::app::components::shell::components::shared::unit_list_search::components::unit_list_search_input::UnitListSearchInput;
use components::filter_trigger::FilterTrigger;
use dioxus::prelude::*;
use model::SearchDialogBarModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SearchDialogBar(props: SearchDialogBarModel) -> Element {
    let value = props.value;
    let placeholder = props.placeholder;
    let on_input = props.on_input;
    let on_keydown = props.on_keydown;
    let active_filter_count = props.active_filter_count;
    let panel_open = props.panel_open;
    let on_toggle_panel = props.on_toggle_panel;
    rsx! {
        div {
            class: CLASS,
            FilterTrigger {
                active: panel_open,
                count: active_filter_count,
                onclick: on_toggle_panel,
            }
            UnitListSearchIcon {}
            UnitListSearchInput {
                value,
                placeholder,
                on_input,
                on_keydown,
            }
        }
    }
}

assert_component!(SearchDialogBar);
