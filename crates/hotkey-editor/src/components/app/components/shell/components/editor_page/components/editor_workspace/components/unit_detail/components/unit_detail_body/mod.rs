pub mod components;
mod props;
mod style;

use components::unit_detail_row::UnitDetailRow;
use dioxus::prelude::*;
pub use props::UnitDetailBodyProps;
use style::CLASS;
use tw_macro::assert_component;
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
