mod logic;
mod props;
mod state;
mod style;

use dioxus::prelude::*;
use logic::GoldHeadingPresentation;
pub use props::GoldHeadingProps;
pub use state::GoldHeadingVariant;
use tw_macro::assert_component;
assert_component!(GoldHeading);

/// The shared uppercase gold heading look — one tracking, gold, drop shadow —
/// varied by [`GoldHeadingVariant`]. A presentational leaf: it inherits its font
/// size (and, for the toast, its tint) from the heading element that wraps it, so
/// each consumer keeps its own element, identity class, and per-band sizing.
#[component]
pub fn GoldHeading(props: GoldHeadingProps) -> Element {
    let GoldHeadingPresentation { class, title } = GoldHeadingPresentation::from(&props);
    rsx! {
        span {
            class,
            {title}
        }
    }
}
