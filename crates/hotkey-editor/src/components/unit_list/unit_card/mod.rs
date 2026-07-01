mod hooks;
mod props;
mod style;
pub mod unit_card_icon;
pub mod unit_card_info;

use crate::assert_component;
use dioxus::prelude::*;
use hooks::use_unit_card;
pub use props::UnitCardProps;
use unit_card_icon::UnitCardIcon;
use unit_card_info::UnitCardInfo;
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
