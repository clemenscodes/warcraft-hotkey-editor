pub mod components;
mod logic;
mod props;
mod style;

use components::unit_id::{UnitId, UnitIdProps};
use components::unit_name_row::{UnitNameRow, UnitNameRowProps};
use dioxus::prelude::*;
pub use props::UnitDetailTitleProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(UnitDetailTitle);

/// The title column of the header: the name row and the unit id.
#[component]
pub fn UnitDetailTitle(props: UnitDetailTitleProps) -> Element {
    let name_row = UnitNameRowProps::from(&props);
    let id = UnitIdProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            UnitNameRow { ..name_row }
            UnitId { ..id }
        }
    }
}
