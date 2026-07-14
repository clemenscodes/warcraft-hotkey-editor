mod model;
mod view;

pub use view::IdleKeycapView;
mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

use model::IdleKeycapModel;

#[component]
pub fn IdleKeycap(props: IdleKeycapModel) -> Element {
    let label = props.label;
    rsx! {
        div {
            class: CLASS,
            {label}
        }
    }
}

assert_component!(IdleKeycap);
