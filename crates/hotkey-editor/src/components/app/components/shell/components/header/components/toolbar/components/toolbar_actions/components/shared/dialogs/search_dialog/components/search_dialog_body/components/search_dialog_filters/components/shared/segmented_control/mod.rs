mod model;
mod view;

pub use view::{SegmentChoice, SegmentedControlView};
mod style;

use crate::components::app::components::shell::components::editor_page::components::shared::toggle_button::ToggleButton;
use dioxus::prelude::*;
use model::SegmentedControlModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SegmentedControl(props: SegmentedControlModel) -> Element {
    let options = props.options;
    rsx! {
        div {
            class: CLASS,
            role: "group",
            for option in options {
                ToggleButton {
                    key: "{option.key}",
                    label: option.label,
                    active: option.is_active,
                    onclick: option.on_pick,
                }
            }
        }
    }
}

assert_component!(SegmentedControl);
