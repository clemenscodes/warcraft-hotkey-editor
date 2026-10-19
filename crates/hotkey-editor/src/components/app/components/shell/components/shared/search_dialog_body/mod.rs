pub mod components;
mod model;
mod presentation;
mod style;
mod view;

pub use view::SearchDialogBodyView;

use crate::components::app::components::shell::components::shared::category_scroll::CategoryScroll;
use components::search_dialog_bar::SearchDialogBar;
use components::search_dialog_filters::SearchDialogFilters;
use components::search_dialog_scrim::SearchDialogScrim;
use dioxus::prelude::*;
use model::SearchDialogBodyModel;
use presentation::use_search_dialog_body;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SearchDialogBody(props: SearchDialogBodyModel) -> Element {
    let SearchDialogBodyModel {} = props;
    let presentation = use_search_dialog_body();
    let search_value = presentation.search_value;
    let search_placeholder = presentation.search_placeholder;
    let on_input = presentation.on_input;
    let on_keydown = presentation.on_keydown;
    let on_clear = presentation.on_clear;
    let groups = presentation.groups;
    let panel_open = presentation.panel_open;
    let panel_mounted = presentation.panel_mounted;
    let panel_leaving = presentation.panel_leaving;
    let active_filter_count = presentation.active_filter_count;
    let on_toggle_panel = presentation.on_toggle_panel;
    let on_dismiss_panel = presentation.on_dismiss_panel;
    rsx! {
        div {
            class: CLASS,
            SearchDialogBar {
                value: search_value,
                placeholder: search_placeholder,
                on_input,
                on_keydown,
                on_clear,
                active_filter_count,
                panel_open,
                on_toggle_panel,
            }
            if panel_mounted {
                SearchDialogFilters {
                    leaving: panel_leaving,
                }
            }
            CategoryScroll {
                groups,
            }
            if panel_mounted {
                SearchDialogScrim {
                    onclick: on_dismiss_panel,
                    leaving: panel_leaving,
                }
            }
        }
    }
}

assert_component!(SearchDialogBody);
