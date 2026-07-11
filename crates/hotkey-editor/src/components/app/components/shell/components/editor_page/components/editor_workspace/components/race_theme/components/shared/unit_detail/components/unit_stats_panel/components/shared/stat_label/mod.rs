mod model;
mod view;

pub use view::StatLabelView;
mod style;

use dioxus::prelude::*;
use model::StatLabelModel;
use style::CLASS;
use tw_macro::assert_component;

/// A stat row's category label in the default treatment: the row's name in the shared
/// gold gilding. Threads only its text; every plain row nests this leaf rather than
/// wearing the label span itself.
#[component]
pub fn StatLabel(props: StatLabelModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(StatLabel);
