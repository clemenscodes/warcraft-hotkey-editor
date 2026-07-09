pub mod components;
mod props;
mod style;

use tw_macro::assert_component;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::conflict_object_id::ConflictObjectId;
use crate::services::navigation::context::use_view_navigation;
use components::conflict_unit_icon::{ConflictUnitIcon, ConflictUnitIconProps};
use components::island_conflict_unit_name::IslandConflictUnitName;
use dioxus::prelude::*;
pub use props::IslandConflictUnitProps;
use style::CLASS;
assert_component!(IslandConflictUnit);

/// The affected unit heading a conflict card: a clickable icon over the name and
/// object id, all deep-linking into the editor focused on that unit through the
/// navigation read from context.
#[component]
pub fn IslandConflictUnit(props: IslandConflictUnitProps) -> Element {
    let name = props.name;
    let unit_id = props.unit_id;
    let view_navigation = use_view_navigation();
    let icon = ConflictUnitIconProps {
        src: props.icon_url,
        alt: name.clone(),
    };
    let onclick = move |_event: MouseEvent| view_navigation.open_unit(unit_id);
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            ConflictUnitIcon { ..icon }
            IslandConflictUnitName { text: name }
            ConflictObjectId { object_id: unit_id }
        }
    }
}
