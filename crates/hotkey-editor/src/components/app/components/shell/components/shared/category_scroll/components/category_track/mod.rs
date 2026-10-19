pub mod components;
mod model;
mod view;

pub use view::CategoryTrackView;
mod style;

use super::super::unit_kind_key;
use components::unit_category_section::UnitCategorySection;
use dioxus::prelude::*;
use model::CategoryTrackModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CategoryTrack(props: CategoryTrackModel) -> Element {
    rsx! {
        div {
            class: CLASS,
            for group in props.groups {
                UnitCategorySection {
                    key: "{unit_kind_key(group.category_kind())}",
                    group,
                }
            }
        }
    }
}

assert_component!(CategoryTrack);
