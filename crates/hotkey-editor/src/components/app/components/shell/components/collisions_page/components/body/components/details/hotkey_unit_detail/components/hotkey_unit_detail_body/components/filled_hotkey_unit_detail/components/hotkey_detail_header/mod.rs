mod model;
mod presentation;
mod view;

pub use view::HotkeyDetailHeaderView;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_detail_unit::ConflictDetailUnit;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_meta::ConflictMeta;
use dioxus::prelude::*;
use model::HotkeyDetailHeaderModel;
use presentation::HotkeyDetailHeaderPresentation;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HotkeyDetailHeader(props: HotkeyDetailHeaderModel) -> Element {
    let HotkeyDetailHeaderPresentation {
        name,
        unit_id,
        icon_url,
        count,
        meta_name,
    } = HotkeyDetailHeaderPresentation::from(&props);
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

assert_component!(HotkeyDetailHeader);
