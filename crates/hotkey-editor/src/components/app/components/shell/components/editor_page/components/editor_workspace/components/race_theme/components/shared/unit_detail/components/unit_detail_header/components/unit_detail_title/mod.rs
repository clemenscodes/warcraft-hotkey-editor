pub mod components;
mod props;
mod view;

pub use view::UnitDetailTitleView;
mod style;

use components::unit_id::UnitId;
use components::unit_name_row::UnitNameRow;
use dioxus::prelude::*;
use props::UnitDetailTitleProps;
use style::CLASS;
use tw_macro::assert_component;

/// The title column of the header: the name row and the unit id.
#[component]
pub fn UnitDetailTitle(props: UnitDetailTitleProps) -> Element {
    let unit_name = props.unit_name;
    let unit_id = props.unit_id;
    let has_hero_attributes = props.has_hero_attributes;
    rsx! {
        div {
            class: CLASS,
            UnitNameRow { unit_name, has_hero_attributes }
            UnitId { unit_id }
        }
    }
}

assert_component!(UnitDetailTitle);
