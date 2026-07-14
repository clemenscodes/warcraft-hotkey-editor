pub mod components;
mod model;
mod view;

pub use view::UnitDetailHeaderView;
mod style;

use components::unit_detail_title::UnitDetailTitle;
use components::unit_portrait::UnitPortrait;
use dioxus::prelude::*;
use model::UnitDetailHeaderModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitDetailHeader(props: UnitDetailHeaderModel) -> Element {
    let portrait_source = props.portrait_url;
    let unit_name = props.unit_name;
    let unit_id = props.unit_id;
    let has_hero_attributes = props.has_hero_attributes;
    rsx! {
        header {
            class: CLASS,
            UnitPortrait {
                src: portrait_source,
                alt: unit_name,
            }
            UnitDetailTitle {
                unit_name,
                unit_id,
                has_hero_attributes,
            }
        }
    }
}

assert_component!(UnitDetailHeader);
