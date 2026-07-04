mod props;
mod state;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::UnitCardIdProps;
use warcraft_api::RaceLabels;
assert_component!(UnitCardId);

/// The unit's database id inside a card.
#[component]
pub fn UnitCardId(props: UnitCardIdProps) -> Element {
    let state = props.state();
    let class = style::class(state);
    let race_attribute = RaceLabels::data_attribute(props.race);
    let text = props.text;
    rsx! {
        code {
            class,
            "data-race": race_attribute,
            {text}
        }
    }
}
