pub mod components;
mod props;
mod style;

use super::shared::stat_icon_frame::{StatIconFrame, StatIconFrameProps};
use components::attribute_rows::{AttributeRows, AttributeRowsProps};
use dioxus::prelude::*;
pub use props::AttributesColumnProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(AttributesColumn);

/// The hero attributes column: the primary-attribute icon beside the three attribute
/// rows, laid into the `attributes` grid area. Present only for a hero unit; an
/// ordinary unit renders nothing here.
#[component]
pub fn AttributesColumn(props: AttributesColumnProps) -> Element {
    let Some(hero) = props.hero else {
        return rsx! {};
    };
    let icon = StatIconFrameProps::from(&hero);
    let rows = AttributeRowsProps::from(&hero);
    rsx! {
        div {
            class: CLASS,
            StatIconFrame { ..icon }
            AttributeRows { ..rows }
        }
    }
}
