pub mod components;
mod logic;
mod props;

use components::inline_conflict_position::{InlineConflictPosition, InlineConflictPositionProps};
use components::top_conflict_position::{TopConflictPosition, TopConflictPositionProps};
use dioxus::prelude::*;
pub use props::ConflictPositionProps;
use tw_macro::assert_component;

/// The colliding command-card cell shown between (or above) a conflict's abilities. A
/// dispatcher: from whether it stacks over a multi-way row it renders
/// `TopConflictPosition` xor `InlineConflictPosition`; there is no `data-top` attribute.
#[component]
pub fn ConflictPosition(props: ConflictPositionProps) -> Element {
    match props.is_top {
        true => {
            let position = TopConflictPositionProps::from(&props);
            rsx! {
                TopConflictPosition { ..position }
            }
        }
        false => {
            let position = InlineConflictPositionProps::from(&props);
            rsx! {
                InlineConflictPosition { ..position }
            }
        }
    }
}

assert_component!(ConflictPosition);
