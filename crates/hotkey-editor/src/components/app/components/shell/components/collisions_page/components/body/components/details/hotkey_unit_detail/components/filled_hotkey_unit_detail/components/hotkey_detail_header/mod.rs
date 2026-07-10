mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_detail_unit::ConflictDetailUnit;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_meta::{
    ConflictMeta, ConflictMetaProps,
};
use dioxus::prelude::*;
pub use props::HotkeyDetailHeaderProps;
use style::CLASS;
use tw_macro::assert_component;

/// The detail-pane header: the selected unit button beside its text meta column.
#[component]
pub fn HotkeyDetailHeader(props: HotkeyDetailHeaderProps) -> Element {
    let meta = ConflictMetaProps::from(&props);
    let unit = props.unit;
    rsx! {
        header {
            class: CLASS,
            ConflictDetailUnit { ..unit }
            ConflictMeta { ..meta }
        }
    }
}

assert_component!(HotkeyDetailHeader);
