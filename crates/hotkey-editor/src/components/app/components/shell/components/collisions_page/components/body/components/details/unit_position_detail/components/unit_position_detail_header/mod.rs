mod components;
mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_detail_unit::ConflictDetailUnit;
use components::unit_position_conflict_meta::{
    UnitPositionConflictMeta, UnitPositionConflictMetaProps,
};
use dioxus::prelude::*;
pub use props::UnitPositionDetailHeaderProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(UnitPositionDetailHeader);

/// The position-collision detail pane header row: the selected unit's icon button
/// beside its text meta column.
#[component]
pub fn UnitPositionDetailHeader(props: UnitPositionDetailHeaderProps) -> Element {
    let meta = UnitPositionConflictMetaProps::from(&props);
    let unit = props.unit;
    rsx! {
        header {
            class: CLASS,
            ConflictDetailUnit { ..unit }
            UnitPositionConflictMeta { ..meta }
        }
    }
}
