pub mod components;
mod model;
mod view;

pub use view::UnitDetailTitleView;
mod style;

use components::unit_id::UnitId;
use components::unit_name_row::UnitNameRow;
use dioxus::prelude::*;
use model::UnitDetailTitleModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitDetailTitle(props: UnitDetailTitleModel) -> Element {
    let unit_name = props.unit_name;
    let unit_id = props.unit_id;
    let has_hero_attributes = props.has_hero_attributes;
    rsx! {
        div {
            class: CLASS,
            UnitNameRow {
                unit_name,
                has_hero_attributes,
            }
            UnitId {
                unit_id,
            }
        }
    }
}

assert_component!(UnitDetailTitle);
