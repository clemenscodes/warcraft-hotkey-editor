pub mod components;
mod hooks;
mod style;

use components::inventory_drag_overlay::InventoryDragOverlay;
use components::system_hotkeys_body::SystemHotkeysBody;
use components::system_hotkeys_breadcrumbs::SystemHotkeysBreadcrumbs;
use dioxus::prelude::*;
use hooks::{SystemHotkeysDialogBodyModel, use_system_hotkeys_dialog_body};
use style::CLASS;
use tw_macro::assert_component;

/// The system-hotkeys dialog's scrolling content region between the header and the
/// panel edge: the category breadcrumbs above the active category's editor, with the
/// inventory drag follower overlaid. A connected host: it reads the dialog UI state
/// from context and feeds its gallery-rendered children their props.
#[component]
pub fn SystemHotkeysDialogBody() -> Element {
    let SystemHotkeysDialogBodyModel {
        active_category,
        drag_follower,
    } = use_system_hotkeys_dialog_body();
    rsx! {
        div {
            class: CLASS,
            SystemHotkeysBreadcrumbs { active_category }
            SystemHotkeysBody {}
            InventoryDragOverlay { drag_follower }
        }
    }
}

assert_component!(SystemHotkeysDialogBody);
