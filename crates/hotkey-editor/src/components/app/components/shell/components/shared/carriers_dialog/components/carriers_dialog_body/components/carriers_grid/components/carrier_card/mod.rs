pub mod components;
mod model;
mod presentation;
mod view;

pub use view::CarrierCardView;
mod style;
use components::carrier_card_icon::CarrierCardIcon;
use components::carrier_card_name::CarrierCardName;
use components::carrier_object_id::CarrierObjectId;
use dioxus::prelude::*;
use model::CarrierCardModel;
use presentation::CarrierCardPresentation;
use presentation::use_carrier_card;
use style::CLASS;
use tw_macro::assert_component;

/// One unit that carries the ability; clicking deep-links into the editor focused on
/// that unit through the navigation read from context.
#[component]
pub fn CarrierCard(props: CarrierCardModel) -> Element {
    let CarrierCardPresentation {
        icon_src,
        icon_alt,
        onclick,
        name,
        unit_id,
    } = use_carrier_card(&props);
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            CarrierCardIcon {
                src: icon_src,
                alt: icon_alt,
            }
            CarrierCardName {
                text: name,
            }
            CarrierObjectId {
                unit_id,
            }
        }
    }
}

assert_component!(CarrierCard);
