pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::unit_card_id::{UnitCardId, UnitCardIdProps};
use components::unit_card_name::{UnitCardName, UnitCardNameProps};
use dioxus::prelude::*;
pub use props::UnitCardInfoProps;
use style::CLASS;
assert_component!(UnitCardInfo);

/// The text column of a unit card: name over database id.
#[component]
pub fn UnitCardInfo(props: UnitCardInfoProps) -> Element {
    let name = UnitCardNameProps::from(&props);
    let id = UnitCardIdProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            UnitCardName { ..name }
            UnitCardId { ..id }
        }
    }
}
