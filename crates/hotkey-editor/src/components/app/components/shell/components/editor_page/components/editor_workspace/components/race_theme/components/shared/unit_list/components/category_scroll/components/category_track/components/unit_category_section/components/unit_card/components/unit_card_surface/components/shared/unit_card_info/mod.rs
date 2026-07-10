pub mod components;
mod props;
mod view;

pub use view::UnitCardInfoView;
mod style;

use components::unit_card_id::UnitCardId;
use components::unit_card_name::UnitCardName;
use dioxus::prelude::*;
use props::UnitCardInfoProps;
use style::CLASS;
use tw_macro::assert_component;

/// The text column of a unit card: name over database id.
#[component]
pub fn UnitCardInfo(props: UnitCardInfoProps) -> Element {
    let display_name = props.display_name.clone();
    let unit_id = props.unit_id;
    let is_selected = props.is_selected;
    rsx! {
        div {
            class: CLASS,
            UnitCardName { text: display_name }
            UnitCardId { unit_id, is_selected }
        }
    }
}

assert_component!(UnitCardInfo);
