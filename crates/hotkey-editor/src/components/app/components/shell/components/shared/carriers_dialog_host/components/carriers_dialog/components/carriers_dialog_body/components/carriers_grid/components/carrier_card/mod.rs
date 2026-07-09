pub mod components;
mod hooks;
mod logic;
mod props;
mod style;
use components::carrier_card_icon::CarrierCardIcon;
use components::carrier_card_name::CarrierCardName;
use components::carrier_object_id::CarrierObjectId;
use dioxus::prelude::*;
use hooks::use_carrier_card;
use logic::CarrierCardModel;
pub use props::CarrierCardProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(CarrierCard);

/// One unit that carries the ability; clicking deep-links into the editor focused on
/// that unit through the navigation read from context.
#[component]
pub fn CarrierCard(props: CarrierCardProps) -> Element {
    let CarrierCardModel {
        icon,
        onclick,
        name,
        unit_id,
    } = use_carrier_card(&props);
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            CarrierCardIcon { ..icon }
            CarrierCardName { text: name }
            CarrierObjectId { unit_id }
        }
    }
}
