pub mod components;
mod props;
mod style;

use crate::assert_component;
use crate::components::views::collisions_page::conflict_object_id::ConflictObjectId;
use components::carrier_card_icon::{CarrierCardIcon, CarrierCardIconProps};
use components::carrier_card_name::CarrierCardName;
use dioxus::prelude::*;
pub use props::CarrierCardProps;
use style::CLASS;
assert_component!(CarrierCard);

/// One unit that carries the shared ability; clicking deep-links into the editor.
#[component]
pub fn CarrierCard(props: CarrierCardProps) -> Element {
    let name = props.name;
    let unit_id_label = props.unit_id.clone();
    let unit_id = props.unit_id;
    let view_navigation = props.view_navigation;
    let icon = props.icon_url.map(|src| CarrierCardIconProps {
        src,
        alt: name.clone(),
    });
    let onclick = move |_event: MouseEvent| view_navigation.open_unit(&unit_id);
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            if let Some(icon) = icon {
                CarrierCardIcon { ..icon }
            }
            CarrierCardName { text: name }
            ConflictObjectId { text: unit_id_label }
        }
    }
}
