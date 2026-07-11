pub mod components;
mod model;
mod view;

pub use view::CategoryTrackView;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_list::unit_kind_key;
use components::unit_category_section::UnitCategorySection;
use dioxus::prelude::*;
use model::CategoryTrackModel;
use style::CLASS;
use tw_macro::assert_component;

/// The inner track laying out the category sections: a vertical stack on the sidebar,
/// a horizontal fixed-height carousel on small screens. It receives the category kinds
/// in display order and renders one section per kind.
#[component]
pub fn CategoryTrack(props: CategoryTrackModel) -> Element {
    rsx! {
        div {
            class: CLASS,
            for category_kind in props.sections {
                UnitCategorySection {
                    key: "{unit_kind_key(category_kind)}",
                    category_kind,
                }
            }
        }
    }
}

assert_component!(CategoryTrack);
