mod props;
mod style;

use dioxus::prelude::*;
pub use props::HitPointsRegenGainProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HitPointsRegenGain);

/// The health-regeneration gain: green, pushed to the row's end, dimmed when the unit
/// does not regenerate. Its shaped text and muted state arrive already resolved.
#[component]
pub fn HitPointsRegenGain(props: HitPointsRegenGainProps) -> Element {
    let text = props.text;
    let is_muted = props.is_muted;
    rsx! {
        span {
            class: CLASS,
            "data-zero": is_muted,
            {text}
        }
    }
}
