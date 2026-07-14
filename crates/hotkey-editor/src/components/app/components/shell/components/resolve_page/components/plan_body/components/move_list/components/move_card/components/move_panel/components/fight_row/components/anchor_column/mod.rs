mod model;
mod presentation;
mod view;

pub use view::AnchorColumnView;
mod style;

use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::ability_icon_host::AbilityIconHost;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::fight_name_plate::FightNamePlate;
use dioxus::prelude::*;
use presentation::AnchorColumnPresentation;
use model::AnchorColumnModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn AnchorColumn(props: AnchorColumnModel) -> Element {
    let move_view = props.move_view;
    let Some(model) = AnchorColumnPresentation::for_move(&move_view) else {
        return rsx! {};
    };
    let AnchorColumnPresentation {
        name,
        object_id,
        icon_url,
        carrier_count,
        is_winner,
        disabled,
        inspected,
    } = model;
    let plate_name = name.clone();
    rsx! {
        div {
            class: CLASS,
            FightNamePlate {
                name: plate_name,
                object_id,
            }
            AbilityIconHost {
                name,
                icon_url,
                carrier_count,
                is_winner,
                disabled,
                inspected,
            }
        }
    }
}

assert_component!(AnchorColumn);
