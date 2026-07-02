pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::unit_detail_row::UnitDetailRow;
use dioxus::prelude::*;
pub use props::UnitDetailBodyProps;
use style::CLASS;
assert_component!(UnitDetailBody);

/// The card body: the grids-and-override row.
#[component]
pub fn UnitDetailBody(props: UnitDetailBodyProps) -> Element {
    rsx! {
        div {
            class: CLASS,
            UnitDetailRow { ..props.row }
        }
    }
}
