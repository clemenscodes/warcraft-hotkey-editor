mod model;
mod view;

mod style;

use dioxus::prelude::*;
use model::MutedManaRegenGainModel;
use style::CLASS;
use tw_macro::assert_component;

/// The muted mana-regeneration look: faint text. Rendered by the
/// [`ManaRegenGain`](super::super::ManaRegenGain) dispatcher when the unit does not
/// regenerate mana.
#[component]
pub fn MutedManaRegenGain(props: MutedManaRegenGainModel) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}

assert_component!(MutedManaRegenGain);
