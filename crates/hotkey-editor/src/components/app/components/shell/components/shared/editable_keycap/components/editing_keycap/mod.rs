mod model;
mod view;

pub use view::EditingKeycapView;
mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

use model::EditingKeycapModel;

#[component]
pub fn EditingKeycap(props: EditingKeycapModel) -> Element {
    let label = props.label;
    rsx! {
        div {
            class: CLASS,
            {label}
        }
    }
}

assert_component!(EditingKeycap);
