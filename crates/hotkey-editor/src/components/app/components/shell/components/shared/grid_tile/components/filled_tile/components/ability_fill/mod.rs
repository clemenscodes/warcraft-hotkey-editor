mod props;
mod style;

use dioxus::prelude::*;
pub use props::AbilityFillProps;
use style::CLASS;
use tw_macro::assert_component;

/// The ability tile's background fill. Rendered only for an ability (or selected)
/// occupant; a command occupant draws `CommandFill` instead.
#[component]
pub fn AbilityFill(props: AbilityFillProps) -> Element {
    if !props.active {
        return rsx! {};
    }
    rsx! {
        div { class: CLASS }
    }
}

assert_component!(AbilityFill);
