pub mod components;
mod model;
mod view;

pub use view::UnitCardIdView;
mod state;

use components::normal_unit_card_id::NormalUnitCardId;
use components::selected_unit_card_id::SelectedUnitCardId;
use dioxus::prelude::*;
use model::UnitCardIdModel;
use state::UnitCardIdState;
use tw_macro::assert_component;

#[component]
pub fn UnitCardId(props: UnitCardIdModel) -> Element {
    let unit_id = props.unit_id;
    match props.state() {
        UnitCardIdState::Normal => rsx! {
            NormalUnitCardId {
                unit_id,
            }
        },
        UnitCardIdState::Selected => rsx! {
            SelectedUnitCardId {
                unit_id,
            }
        },
    }
}

assert_component!(UnitCardId);
