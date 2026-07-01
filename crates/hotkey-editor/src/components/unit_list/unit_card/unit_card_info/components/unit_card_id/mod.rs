mod props;
mod state;
mod style;

use dioxus::prelude::*;

use crate::assert_component;

pub use props::UnitCardIdProps;

assert_component!(UnitCardId);

/// The unit's database id inside a card.
#[component]
pub fn UnitCardId(props: UnitCardIdProps) -> Element {
    let state = props.state();
    let class = style::class(state);
    let text = props.text;
    rsx! {
        code { class, {text} }
    }
}
