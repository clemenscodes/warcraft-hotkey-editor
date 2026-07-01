pub mod components;
mod logic;
mod props;
mod style;

use crate::assert_component;
use components::hero_level_picker::{HeroLevelPicker, HeroLevelPickerProps};
use components::unit_name::{UnitName, UnitNameProps};
use dioxus::prelude::*;
pub use props::UnitNameRowProps;
use style::CLASS;
assert_component!(UnitNameRow);

/// The unit name beside the optional hero level picker.
#[component]
pub fn UnitNameRow(props: UnitNameRowProps) -> Element {
    let name = UnitNameProps::from(&props);
    let picker = HeroLevelPickerProps::from(&props);
    let has_hero_attributes = props.has_hero_attributes;
    rsx! {
        div {
            class: CLASS,
            UnitName { ..name }
            if has_hero_attributes {
                HeroLevelPicker { ..picker }
            }
        }
    }
}
