pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::unit_card_id::UnitCardId;
use components::unit_card_name::UnitCardName;
use dioxus::prelude::*;
pub use props::UnitCardInfoProps;
use style::CLASS;
assert_component!(UnitCardInfo);

/// The text column of a unit card: name over database id.
#[component]
pub fn UnitCardInfo(props: UnitCardInfoProps) -> Element {
    let display_name = props.display_name;
    let unit_id = props.unit_id;
    let is_selected = props.is_selected;
    rsx! {
        div { class: CLASS,
            UnitCardName { text: display_name, is_selected }
            UnitCardId { text: unit_id, is_selected }
        }
    }
}
