pub mod components;
mod presentation;
mod style;

use components::pager_track::PagerTrack;
use dioxus::prelude::*;
use presentation::{MobileEditorPresentation, use_mobile_editor};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn MobileEditor() -> Element {
    let presentation = use_mobile_editor();
    let MobileEditorPresentation {
        onmounted,
        window_unit_ids,
    } = presentation;
    rsx! {
        section {
            class: CLASS,
            aria_label: "Mobile editor",
            tabindex: "-1",
            onmounted: move |event| onmounted.call(event),
            PagerTrack {
                unit_ids: window_unit_ids,
            }
        }
    }
}

assert_component!(MobileEditor);
