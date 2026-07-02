pub mod components;
mod hooks;
mod props;
mod style;

use crate::assert_component;
use components::unit_card_icon::{UnitCardIcon, UnitCardIconProps};
use components::unit_card_info::{UnitCardInfo, UnitCardInfoProps};
use dioxus::prelude::*;
use hooks::use_unit_card;
pub use props::UnitCardProps;
assert_component!(UnitCard);

/// One selectable unit in the list: portrait plus name and id. Selecting it drives
/// the unit-detail panel.
#[component]
pub fn UnitCard(props: UnitCardProps) -> Element {
    let model = use_unit_card(&props);
    let icon = UnitCardIconProps::from(&props);
    let info = UnitCardInfoProps::from(&props);
    let is_selected = props.is_selected;
    rsx! {
        button {
            class: model.class,
            "data-unit-kind": model.kind_attr,
            "data-selected": is_selected,
            onclick: model.on_click,
            onkeydown: model.on_keydown,
            UnitCardIcon { ..icon }
            UnitCardInfo { ..info }
        }
    }
}
