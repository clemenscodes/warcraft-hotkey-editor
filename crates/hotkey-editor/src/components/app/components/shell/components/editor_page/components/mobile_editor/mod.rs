pub mod components;
mod presentation;
mod state;
mod style;

use components::pager_card_host::PagerCardHost;
use dioxus::prelude::*;
use presentation::{MobileEditorPresentation, use_mobile_editor};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn MobileEditor() -> Element {
    let presentation = use_mobile_editor();
    let MobileEditorPresentation {
        onmounted,
        onscroll,
        onscrollend,
        onpointerdown,
        onkeydown,
        window_unit_ids,
    } = presentation;
    rsx! {
        section {
            class: CLASS,
            aria_label: "Mobile editor",
            onmounted: move |event| onmounted.call(event),
            onscroll: move |event| onscroll.call(event),
            onscrollend: move |event| onscrollend.call(event),
            onpointerdown: move |event| onpointerdown.call(event),
            onkeydown: move |event| onkeydown.call(event),
            for unit_id in window_unit_ids {
                PagerCardHost {
                    key: "{unit_id}",
                    unit_id,
                }
            }
        }
    }
}

assert_component!(MobileEditor);
