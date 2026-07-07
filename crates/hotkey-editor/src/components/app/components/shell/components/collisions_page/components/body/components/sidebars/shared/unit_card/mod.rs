pub mod components;
mod props;
mod style;

use components::hotkey_unit_row_icon::HotkeyUnitRowIcon;
use tw_macro::assert_component;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::collision_count::CollisionCount;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::conflict_object_id::ConflictObjectId;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::hotkey_unit_name::HotkeyUnitName;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::row_meta::RowMeta;
use dioxus::prelude::*;
pub use props::UnitCardProps;
use style::CLASS;
assert_component!(UnitCard);

/// A selectable unit-collision card: portrait plus the name, object id, and
/// collision-count meta.
#[component]
pub fn UnitCard(props: UnitCardProps) -> Element {
    let is_selected = props.is_selected;
    let collision_key = props.collision_key;
    let onclick = props.onclick;
    let name = props.name;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "data-selected": is_selected,
            "data-collision-key": collision_key,
            onclick,
            HotkeyUnitRowIcon { icon_url: props.icon_url, alt: name.clone() }
            RowMeta {
                HotkeyUnitName { text: name }
                ConflictObjectId { text: props.unit_id }
                CollisionCount { count: props.count }
            }
        }
    }
}
