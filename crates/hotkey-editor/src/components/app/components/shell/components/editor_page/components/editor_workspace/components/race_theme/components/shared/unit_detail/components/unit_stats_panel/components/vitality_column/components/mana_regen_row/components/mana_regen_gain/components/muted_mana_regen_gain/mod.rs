mod props;
mod style;

use dioxus::prelude::*;
pub use props::MutedManaRegenGainProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(MutedManaRegenGain);

/// The muted mana-regeneration look: faint text. Rendered by the
/// [`ManaRegenGain`](super::super::ManaRegenGain) dispatcher when the unit does not
/// regenerate mana.
#[component]
pub fn MutedManaRegenGain(props: MutedManaRegenGainProps) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}
