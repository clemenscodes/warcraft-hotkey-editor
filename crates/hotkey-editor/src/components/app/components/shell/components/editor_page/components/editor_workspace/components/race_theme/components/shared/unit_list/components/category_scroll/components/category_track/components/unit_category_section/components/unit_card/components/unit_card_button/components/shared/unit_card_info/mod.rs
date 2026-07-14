pub mod components;
mod model;
mod view;

pub use view::UnitCardInfoView;
mod style;

use components::unit_card_id::UnitCardId;
use components::unit_card_name::UnitCardName;
use dioxus::prelude::*;
use model::UnitCardInfoModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitCardInfo(props: UnitCardInfoModel) -> Element {
    let display_name = props.display_name.clone();
    let unit_id = props.unit_id;
    let is_selected = props.is_selected;
    rsx! {
        div {
            class: CLASS,
            UnitCardName {
                text: display_name,
            }
            UnitCardId {
                unit_id,
                is_selected,
            }
        }
    }
}

assert_component!(UnitCardInfo);
