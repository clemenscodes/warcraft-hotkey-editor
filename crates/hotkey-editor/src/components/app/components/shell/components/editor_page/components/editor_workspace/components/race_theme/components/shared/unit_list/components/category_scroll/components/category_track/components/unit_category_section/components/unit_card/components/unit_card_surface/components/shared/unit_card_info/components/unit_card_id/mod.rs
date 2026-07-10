pub mod components;
mod props;
mod state;

use components::normal_unit_card_id::NormalUnitCardId;
use components::selected_unit_card_id::SelectedUnitCardId;
use dioxus::prelude::*;
use props::UnitCardIdProps;
use state::UnitCardIdState;
use tw_macro::assert_component;

/// The unit's database id inside a card. A pure dispatcher: from the card's selected
/// flag it renders the muted `NormalUnitCardId` xor the race-accented
/// `SelectedUnitCardId`. Each look owns its own classed `code` root; this dispatcher
/// only forwards the unit id to the one the state selects.
#[component]
pub fn UnitCardId(props: UnitCardIdProps) -> Element {
    let unit_id = props.unit_id;
    match props.state() {
        UnitCardIdState::Normal => rsx! {
            NormalUnitCardId { unit_id }
        },
        UnitCardIdState::Selected => rsx! {
            SelectedUnitCardId { unit_id }
        },
    }
}

assert_component!(UnitCardId);
