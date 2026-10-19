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
        window_pages,
    } = presentation;
    rsx! {
        section {
            class: CLASS,
            aria_label: "Mobile editor",
            tabindex: "-1",
            onmounted: move |event| onmounted.call(event),
            PagerTrack {
                pages: window_pages,
            }
        }
    }
}

assert_component!(MobileEditor);
