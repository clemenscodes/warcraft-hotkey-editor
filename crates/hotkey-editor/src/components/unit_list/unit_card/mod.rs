pub mod unit_card_icon;
pub mod unit_card_info;
mod hooks;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use hooks::use_unit_card;
use unit_card_icon::UnitCardIcon;
use unit_card_info::UnitCardInfo;

pub use props::UnitCardProps;

assert_component!(UnitCard);

/// One selectable unit in the list: portrait plus name and id. Selecting it drives
/// the unit-detail panel.
#[component]
pub fn UnitCard(props: UnitCardProps) -> Element {
    let model = use_unit_card(&props);
    let display_name = props.display_name;
    let unit_id = props.unit_id;
    let icon_path = props.icon_path;
    let is_selected = props.is_selected;
    let display_name_for_icon = display_name.clone();
    rsx! {
        button {
            class: model.class,
            "data-unit-kind": model.kind_attr,
            "data-selected": is_selected,
            onclick: model.on_click,
            onkeydown: model.on_keydown,
            UnitCardIcon { icon_path, display_name: display_name_for_icon }
            UnitCardInfo { display_name, unit_id, is_selected }
        }
    }
}
