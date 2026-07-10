mod props;
mod style;

use dioxus::prelude::*;
pub use props::StatLabelProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(StatLabel);

/// A stat row's category label in the default treatment: the row's name in the shared
/// gold gilding. Threads only its text; every plain row nests this leaf rather than
/// wearing the label span itself.
#[component]
pub fn StatLabel(props: StatLabelProps) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}
