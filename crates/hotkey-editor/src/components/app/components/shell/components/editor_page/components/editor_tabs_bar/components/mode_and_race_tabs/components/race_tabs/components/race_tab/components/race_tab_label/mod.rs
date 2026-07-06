mod props;
mod style;

use dioxus::prelude::*;
pub use props::RaceTabLabelProps;
use tw_macro::assert_component;
assert_component!(RaceTabLabel);

/// The race name overlaid on the banner. Its active color is the race accent,
/// driven by the parent button's `group` active state.
#[component]
pub fn RaceTabLabel(props: RaceTabLabelProps) -> Element {
    let class = style::class(props.race);
    let label = props.label;
    rsx! {
        span {
            class,
            {label}
        }
    }
}
