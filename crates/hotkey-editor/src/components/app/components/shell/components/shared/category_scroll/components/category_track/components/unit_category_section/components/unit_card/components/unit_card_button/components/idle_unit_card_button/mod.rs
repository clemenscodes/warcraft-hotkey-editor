mod model;
mod view;

pub use view::IdleUnitCardButtonView;
mod style;

use crate::components::app::components::shell::components::shared::category_scroll::components::category_track::components::unit_category_section::components::unit_card::components::unit_card_button::components::shared::unit_card_icon::UnitCardIcon;
use crate::components::app::components::shell::components::shared::category_scroll::components::category_track::components::unit_category_section::components::unit_card::components::unit_card_button::components::shared::unit_card_info::UnitCardInfo;
use dioxus::prelude::*;
use model::IdleUnitCardButtonModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn IdleUnitCardButton(props: IdleUnitCardButtonModel) -> Element {
    let icon_path = props.icon_path.clone();
    let display_name = props.display_name.clone();
    let icon_display_name = display_name.clone();
    let unit_id = props.unit_id;
    let is_selected = false;
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            onkeydown,
            UnitCardIcon {
                icon_path,
                display_name: icon_display_name,
            }
            UnitCardInfo {
                display_name,
                unit_id,
                is_selected,
            }
        }
    }
}

assert_component!(IdleUnitCardButton);
