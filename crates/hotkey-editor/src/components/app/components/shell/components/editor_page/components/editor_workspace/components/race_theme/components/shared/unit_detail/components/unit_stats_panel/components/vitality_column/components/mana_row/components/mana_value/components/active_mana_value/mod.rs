mod props;
mod style;

use dioxus::prelude::*;
use props::ActiveManaValueProps;
use style::CLASS;
use tw_macro::assert_component;

/// The active mana look: the human-blue accent, semibold and enlarged. Rendered by the
/// [`ManaValue`](super::super::ManaValue) dispatcher when the unit has a mana pool.
#[component]
pub fn ActiveManaValue(props: ActiveManaValueProps) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}

assert_component!(ActiveManaValue);
