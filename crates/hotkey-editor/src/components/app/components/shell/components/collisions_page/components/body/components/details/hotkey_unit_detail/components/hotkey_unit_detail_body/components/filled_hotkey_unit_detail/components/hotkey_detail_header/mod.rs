mod model;
mod view;

pub use view::HotkeyDetailHeaderView;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_detail_unit::ConflictDetailUnit;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_meta::ConflictMeta;
use dioxus::prelude::*;
use model::HotkeyDetailHeaderModel;
use style::CLASS;
use tw_macro::assert_component;

/// The detail-pane header: the selected unit button beside its text meta column.
#[component]
pub fn HotkeyDetailHeader(props: HotkeyDetailHeaderModel) -> Element {
    let name = props.unit.name().to_owned();
    let unit_id = props.unit.unit_id();
    let icon_url = props.unit.icon_url().map(str::to_owned);
    let count = props.count;
    let meta_name = name.clone();
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
