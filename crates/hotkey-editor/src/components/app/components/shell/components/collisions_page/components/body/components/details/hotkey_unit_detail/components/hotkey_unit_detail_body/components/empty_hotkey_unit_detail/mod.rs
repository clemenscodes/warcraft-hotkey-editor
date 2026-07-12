mod model;
mod style;
mod view;

pub use view::EmptyHotkeyUnitDetailView;

use dioxus::prelude::*;
use model::EmptyHotkeyUnitDetailModel;
use style::CLASS;
use tw_macro::assert_component;

/// The empty detail pane: the base pane surface, centered and muted, showing the prompt.
#[component]
pub fn EmptyHotkeyUnitDetail(props: EmptyHotkeyUnitDetailModel) -> Element {
    rsx! {
        div {
            class: CLASS,
            p { {props.prompt} }
        }
    }
}

assert_component!(EmptyHotkeyUnitDetail);
