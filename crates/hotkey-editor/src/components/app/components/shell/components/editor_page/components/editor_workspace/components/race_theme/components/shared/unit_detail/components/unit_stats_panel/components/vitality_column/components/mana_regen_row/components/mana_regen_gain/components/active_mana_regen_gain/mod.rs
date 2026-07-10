mod props;
mod style;

use dioxus::prelude::*;
use props::ActiveManaRegenGainProps;
use style::CLASS;
use tw_macro::assert_component;

/// The active mana-regeneration look: the human-blue accent. Rendered by the
/// [`ManaRegenGain`](super::super::ManaRegenGain) dispatcher when the unit regenerates
/// mana.
#[component]
pub fn ActiveManaRegenGain(props: ActiveManaRegenGainProps) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}

assert_component!(ActiveManaRegenGain);
