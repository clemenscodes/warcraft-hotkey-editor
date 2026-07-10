pub mod components;
mod props;
mod style;

use components::race_tab_label::RaceTabLabel;
use dioxus::prelude::*;
pub use props::RaceTabProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(RaceTab);

/// The base race tab: the banner button itself, and the single owner of the race-tab
/// look. It is the most-nested leaf — its `style.rs` is a private `mod style` that
/// nothing extends and nothing re-exports, so the look lives here once and cannot leak.
/// Richer looks reuse it by composition: the active variant renders `RaceTab` and adds
/// its accent on top; the inactive variant renders it as-is. The banner colour, image,
/// hover accent, and label colour all come from custom properties, so this button is the
/// same for every race and both states.
#[component]
pub fn RaceTab(props: RaceTabProps) -> Element {
    let label = props.label;
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    rsx! {
        button {
            class: CLASS,
            onclick,
            onkeydown,
            RaceTabLabel { ..label }
        }
    }
}
