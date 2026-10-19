mod data;
mod presentation;
mod style;

use super::shared::filter_group_label::FilterGroupLabel;
use super::shared::segmented_control::SegmentedControl;
use dioxus::prelude::*;
use presentation::use_mode_group;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ModeGroup() -> Element {
    let options = use_mode_group();
    rsx! {
        div {
            class: CLASS,
            FilterGroupLabel {
                label: data::LABEL,
            }
            SegmentedControl {
                options,
            }
        }
    }
}

assert_component!(ModeGroup);
