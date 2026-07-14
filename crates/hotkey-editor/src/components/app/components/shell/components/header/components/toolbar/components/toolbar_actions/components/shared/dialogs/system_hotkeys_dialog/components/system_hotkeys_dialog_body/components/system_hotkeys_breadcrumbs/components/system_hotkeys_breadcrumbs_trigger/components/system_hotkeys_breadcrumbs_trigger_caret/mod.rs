pub mod components;
mod model;
mod view;

pub use view::SystemHotkeysBreadcrumbsTriggerCaretView;

use components::closed_breadcrumbs_trigger_caret::ClosedBreadcrumbsTriggerCaret;
use components::open_breadcrumbs_trigger_caret::OpenBreadcrumbsTriggerCaret;
use dioxus::prelude::*;
use model::SystemHotkeysBreadcrumbsTriggerCaretModel;
use tw_macro::assert_component;

/// The trigger's caret glyph. A pure dispatcher: from the dropdown's open flag it
/// renders the flipped look (`OpenBreadcrumbsTriggerCaret`) xor the resting look
/// (`ClosedBreadcrumbsTriggerCaret`).
#[component]
pub fn SystemHotkeysBreadcrumbsTriggerCaret(
    props: SystemHotkeysBreadcrumbsTriggerCaretModel,
) -> Element {
    if props.is_open {
        rsx! {
            OpenBreadcrumbsTriggerCaret {
            


            }
        }
    } else {
        rsx! {
            ClosedBreadcrumbsTriggerCaret {
            


            }
        }
    }
}

assert_component!(SystemHotkeysBreadcrumbsTriggerCaret);
