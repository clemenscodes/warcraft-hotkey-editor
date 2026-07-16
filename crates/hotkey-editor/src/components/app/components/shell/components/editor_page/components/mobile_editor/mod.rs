pub mod components;
mod presentation;
mod style;

use components::pager_card_host::PagerCardHost;
use components::pager_spacer::PagerSpacer;
use dioxus::prelude::*;
use presentation::{MobileEditorPresentation, use_mobile_editor};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn MobileEditor() -> Element {
    let presentation = use_mobile_editor();
    let MobileEditorPresentation {
        onmounted,
        onscrollend,
        top_spacer_px,
        bottom_spacer_px,
        window_unit_ids,
    } = presentation;
    rsx! {
        section {
            class: CLASS,
            aria_label: "Mobile editor",
            onmounted: move |event| onmounted.call(event),
            onscrollend: move |event| onscrollend.call(event),
            PagerSpacer {
                height_px: top_spacer_px,
            }
            for unit_id in window_unit_ids {
                PagerCardHost {
                    key: "{unit_id}",
                    unit_id,
                }
            }
            PagerSpacer {
                height_px: bottom_spacer_px,
            }
        }
    }
}

assert_component!(MobileEditor);
