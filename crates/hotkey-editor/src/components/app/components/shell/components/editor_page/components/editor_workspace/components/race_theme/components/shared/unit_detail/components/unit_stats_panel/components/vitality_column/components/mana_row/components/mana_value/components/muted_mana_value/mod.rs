mod props;
mod view;

mod style;

use dioxus::prelude::*;
use props::MutedManaValueProps;
use style::CLASS;
use tw_macro::assert_component;

/// The muted mana look: faint, normal weight. Rendered by the
/// [`ManaValue`](super::super::ManaValue) dispatcher when the unit has no mana pool.
#[component]
pub fn MutedManaValue(props: MutedManaValueProps) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}

assert_component!(MutedManaValue);
