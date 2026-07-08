pub mod components;
mod props;
mod style;

use components::inventory_drag_overlay::{InventoryDragOverlay, InventoryDragOverlayProps};
use components::system_hotkeys_body::{SystemHotkeysBody, SystemHotkeysBodyProps};
use components::system_hotkeys_breadcrumbs::{
    SystemHotkeysBreadcrumbs, SystemHotkeysBreadcrumbsProps,
};
use dioxus::prelude::*;
pub use props::SystemHotkeysDialogBodyProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(SystemHotkeysDialogBody);

/// The system-hotkeys dialog's scrolling content region between the header and the
/// panel edge: the category breadcrumbs above the active category's editor, with the
/// inventory drag follower overlaid.
#[component]
pub fn SystemHotkeysDialogBody(props: SystemHotkeysDialogBodyProps) -> Element {
    let breadcrumbs = SystemHotkeysBreadcrumbsProps::from(&props);
    let body = SystemHotkeysBodyProps::from(&props);
    let overlay = InventoryDragOverlayProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            SystemHotkeysBreadcrumbs { ..breadcrumbs }
            SystemHotkeysBody { ..body }
            InventoryDragOverlay { ..overlay }
        }
    }
}
