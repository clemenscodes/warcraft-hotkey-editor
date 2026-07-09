mod props;
mod style;

use dioxus::prelude::*;
pub use props::RegenLabelProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(RegenLabel);

/// A regeneration row's label: a dimmer, smaller gold than the headline rows, marking
/// the indented companion beneath a vitality figure.
#[component]
pub fn RegenLabel(props: RegenLabelProps) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}
