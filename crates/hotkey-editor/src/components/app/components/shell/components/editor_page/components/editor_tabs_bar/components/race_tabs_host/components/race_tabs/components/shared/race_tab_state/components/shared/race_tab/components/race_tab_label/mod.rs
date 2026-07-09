mod props;
mod style;

use dioxus::prelude::*;
pub use props::RaceTabLabelProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(RaceTabLabel);

/// The race name overlaid on the banner. Its colour is `--label-color`, defaulting to
/// white; the active tab publishes its race accent into that variable, so the label
/// turns accent-coloured when its tab is active and is white otherwise — one label for
/// every race and state, coloured by the button, never a per-state branch here.
#[component]
pub fn RaceTabLabel(props: RaceTabLabelProps) -> Element {
    let label = props.label;
    rsx! {
        span {
            class: CLASS,
            {label}
        }
    }
}
