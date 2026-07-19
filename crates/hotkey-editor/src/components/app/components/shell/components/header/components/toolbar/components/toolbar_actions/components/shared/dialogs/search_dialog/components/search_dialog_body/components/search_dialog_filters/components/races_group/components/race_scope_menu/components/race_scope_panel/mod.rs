pub mod components;
mod model;
mod presentation;
mod style;
mod view;

pub use view::RaceScopePanelView;

use components::race_scope_back_button::RaceScopeBackButton;
use components::race_scope_badge::RaceScopeBadge;
use dioxus::prelude::*;
use model::RaceScopePanelModel;
use presentation::use_race_scope_panel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn RaceScopePanel(props: RaceScopePanelModel) -> Element {
    let on_back = props.on_back;
    let race_choices = use_race_scope_panel();
    rsx! {
        div {
            class: CLASS,
            role: "group",
            aria_label: "Races to search",
            RaceScopeBackButton {
                onclick: on_back,
            }
            for choice in race_choices {
                RaceScopeBadge {
                    key: "{choice.key}",
                    race: choice.race,
                    is_active: choice.is_active,
                    label: choice.label,
                    onclick: choice.onclick,
                    onkeydown: choice.onkeydown,
                }
            }
        }
    }
}

assert_component!(RaceScopePanel);
