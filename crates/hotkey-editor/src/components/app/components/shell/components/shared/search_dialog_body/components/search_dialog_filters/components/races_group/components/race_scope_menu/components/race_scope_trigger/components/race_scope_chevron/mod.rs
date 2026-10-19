pub mod components;
mod model;
mod view;

pub use view::RaceScopeChevronView;

use components::closed_race_scope_chevron::ClosedRaceScopeChevron;
use components::open_race_scope_chevron::OpenRaceScopeChevron;
use dioxus::prelude::*;
use model::RaceScopeChevronModel;
use tw_macro::assert_component;

#[component]
pub fn RaceScopeChevron(props: RaceScopeChevronModel) -> Element {
    match props.is_open {
        true => rsx! {
            OpenRaceScopeChevron {}
        },
        false => rsx! {
            ClosedRaceScopeChevron {}
        },
    }
}

assert_component!(RaceScopeChevron);
