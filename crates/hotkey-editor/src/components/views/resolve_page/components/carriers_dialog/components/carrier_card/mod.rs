pub mod components;
mod props;
mod style;
use crate::assert_component;
use components::carrier_card_icon::{CarrierCardIcon, CarrierCardIconProps};
use components::carrier_card_name::CarrierCardName;
use components::carrier_object_id::CarrierObjectId;
use dioxus::prelude::*;
pub use props::CarrierCardProps;
use style::CLASS;
assert_component!(CarrierCard);
#[component]
pub fn CarrierCard(props: CarrierCardProps) -> Element {
    let name = props.name;
    let unit_id_label = props.unit_id.clone();
    let unit_id = props.unit_id;
    let view_navigation = props.view_navigation;
    let icon = CarrierCardIconProps {
        src: props.icon_url,
        alt: name.clone(),
    };
    let onclick = move |_event: MouseEvent| view_navigation.open_unit(&unit_id);
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            CarrierCardIcon { ..icon }
            CarrierCardName { text: name }
            CarrierObjectId { text: unit_id_label }
        }
    }
}
