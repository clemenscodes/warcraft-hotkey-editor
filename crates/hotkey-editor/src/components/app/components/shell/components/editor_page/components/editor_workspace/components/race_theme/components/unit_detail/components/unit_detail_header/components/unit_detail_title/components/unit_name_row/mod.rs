pub mod components;
mod model;
mod view;

pub use view::UnitNameRowView;
mod style;

use components::hero_level_picker::HeroLevelPicker;
use components::unit_name::UnitName;
use dioxus::prelude::*;
use model::UnitNameRowModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitNameRow(props: UnitNameRowModel) -> Element {
    let unit_name = props.unit_name;
    let has_hero_attributes = props.has_hero_attributes;
    rsx! {
        div {
            class: CLASS,
            UnitName {
                text: unit_name,
            }
            if has_hero_attributes {
                HeroLevelPicker {}
            }
        }
    }
}

assert_component!(UnitNameRow);
