pub mod components;
mod logic;
mod props;
mod state;

use components::normal_unit_card_id::{NormalUnitCardId, NormalUnitCardIdProps};
use components::selected_unit_card_id::{SelectedUnitCardId, SelectedUnitCardIdProps};
use dioxus::prelude::*;
pub use props::UnitCardIdProps;
use state::UnitCardIdState;
use tw_macro::assert_component;

/// The unit's database id inside a card. A pure dispatcher: from the card's selected
/// flag it renders the muted `NormalUnitCardId` xor the race-accented
/// `SelectedUnitCardId`. Each look owns its own classed `code` root; this dispatcher
/// only builds each look's props and renders the one the state selects.
#[component]
pub fn UnitCardId(props: UnitCardIdProps) -> Element {
    match props.state() {
        UnitCardIdState::Normal => {
            let id = NormalUnitCardIdProps::from(&props);
            rsx! {
                NormalUnitCardId { ..id }
            }
        }
        UnitCardIdState::Selected => {
            let id = SelectedUnitCardIdProps::from(&props);
            rsx! {
                SelectedUnitCardId { ..id }
            }
        }
    }
}

assert_component!(UnitCardId);
