pub mod components;
mod model;
mod view;

pub use view::UnitDetailHeaderView;
mod style;

use components::unit_detail_title::UnitDetailTitle;
use crate::components::app::components::shell::components::editor_page::components::shared::unit_detail_trigger::UnitDetailTrigger;
use dioxus::prelude::*;
use model::UnitDetailHeaderModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitDetailHeader(props: UnitDetailHeaderModel) -> Element {
    let icon_url = props.portrait_url;
    let unit_name = props.unit_name;
    let unit_id = props.unit_id;
    let has_hero_attributes = props.has_hero_attributes;
    rsx! {
        header {
            class: CLASS,
            UnitDetailTrigger {
                icon_url,
                unit_id,
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
