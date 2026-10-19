mod model;
mod presentation;
mod view;

pub use view::UnitConflictDetailHeaderView;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_detail_unit::ConflictDetailUnit;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_meta::ConflictMeta;
use dioxus::prelude::*;
use model::UnitConflictDetailHeaderModel;
use presentation::UnitConflictDetailHeaderPresentation;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitConflictDetailHeader(props: UnitConflictDetailHeaderModel) -> Element {
    let UnitConflictDetailHeaderPresentation {
        name,
        unit_id,
        icon_url,
        count,
        meta_name,
    } = UnitConflictDetailHeaderPresentation::from(&props);
    rsx! {
        header {
            class: CLASS,
            ConflictDetailUnit {
                unit_id,
                icon_url,
                name,
            }
            ConflictMeta {
                name: meta_name,
                unit_id,
                count,
            }
        }
    }
}

assert_component!(UnitConflictDetailHeader);
