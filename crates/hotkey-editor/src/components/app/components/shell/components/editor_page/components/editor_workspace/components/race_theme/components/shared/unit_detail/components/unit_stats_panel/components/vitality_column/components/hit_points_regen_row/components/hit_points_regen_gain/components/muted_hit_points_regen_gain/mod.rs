mod props;
mod view;

mod style;

use dioxus::prelude::*;
use props::MutedHitPointsRegenGainProps;
use style::CLASS;
use tw_macro::assert_component;

/// The muted health-regeneration look: faint text. Rendered by the
/// [`HitPointsRegenGain`](super::super::HitPointsRegenGain) dispatcher when the unit
/// does not regenerate health.
#[component]
pub fn MutedHitPointsRegenGain(props: MutedHitPointsRegenGainProps) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}

assert_component!(MutedHitPointsRegenGain);
