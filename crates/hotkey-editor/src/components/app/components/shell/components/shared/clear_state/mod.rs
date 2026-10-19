pub mod components;
mod model;
mod style;
mod view;

pub use view::ClearStateView;

use components::clear_icon::ClearIcon;
use components::clear_label::ClearLabel;
use dioxus::prelude::*;
use model::ClearStateModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ClearState(props: ClearStateModel) -> Element {
    let text = props.text;
    rsx! {
        section {
            class: CLASS,
            ClearIcon {}
            ClearLabel {
                text,
            }
        }
    }
}

assert_component!(ClearState);
