mod components;
mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_detail_unit::ConflictDetailUnit;
use components::hotkey_conflict_meta::{HotkeyConflictMeta, HotkeyConflictMetaProps};
use dioxus::prelude::*;
pub use props::HotkeyDetailHeaderProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HotkeyDetailHeader);

/// The detail-pane header: the selected unit button beside its text meta column.
#[component]
pub fn HotkeyDetailHeader(props: HotkeyDetailHeaderProps) -> Element {
    let meta = HotkeyConflictMetaProps::from(&props);
    let unit = props.unit;
    rsx! {
        header {
            class: CLASS,
            ConflictDetailUnit { ..unit }
            HotkeyConflictMeta { ..meta }
        }
    }
}
