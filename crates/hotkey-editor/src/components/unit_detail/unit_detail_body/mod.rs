mod props;
mod style;

use super::unit_detail_row::UnitDetailRow;
use crate::assert_component;
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
