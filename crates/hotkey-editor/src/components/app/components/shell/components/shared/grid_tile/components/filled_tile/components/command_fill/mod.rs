mod model;
mod view;

pub use view::CommandFillView;
mod style;

use dioxus::prelude::*;
use model::CommandFillModel;
use style::CLASS;
use tw_macro::assert_component;

/// The command tile's background fill. Rendered only for a built-in command
/// occupant; an ability occupant draws `AbilityFill` instead.
#[component]
pub fn CommandFill(props: CommandFillModel) -> Element {
    if !props.active {
        return rsx! {};
    }
    rsx! {
        div {
            class: CLASS,
        }
    }
}

assert_component!(CommandFill);
