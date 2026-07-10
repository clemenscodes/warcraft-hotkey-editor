pub mod components;
mod props;
mod style;

use components::unit_category_section::UnitCategorySection;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_list::unit_kind_data_attr;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

pub use props::CategoryTrackProps;

assert_component!(CategoryTrack);

/// The inner track laying out the category sections: a vertical stack on the sidebar,
/// a horizontal fixed-height carousel on small screens. The sections arrive already
/// shaped as props.
#[component]
pub fn CategoryTrack(props: CategoryTrackProps) -> Element {
    rsx! {
        div {
            class: CLASS,
            for section in props.sections {
                UnitCategorySection {
                    key: "{unit_kind_data_attr(section.category_kind)}",
                    ..section
                }
            }
        }
    }
}
