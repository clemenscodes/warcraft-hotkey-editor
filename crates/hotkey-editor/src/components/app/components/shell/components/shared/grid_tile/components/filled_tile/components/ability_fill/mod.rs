mod model;
mod view;

pub use view::AbilityFillView;
mod style;

use dioxus::prelude::*;
use model::AbilityFillModel;
use style::CLASS;
use tw_macro::assert_component;

/// The ability tile's background fill. Rendered only for an ability (or selected)
/// occupant; a command occupant draws `CommandFill` instead.
#[component]
pub fn AbilityFill(props: AbilityFillModel) -> Element {
    if !props.active {
        return rsx! {};
    }
    rsx! {
        div {
            class: CLASS,
        }
    }
}

assert_component!(AbilityFill);
