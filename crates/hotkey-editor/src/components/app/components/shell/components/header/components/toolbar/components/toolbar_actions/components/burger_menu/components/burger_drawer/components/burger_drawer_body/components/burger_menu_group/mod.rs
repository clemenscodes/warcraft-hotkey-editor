pub mod components;
mod model;
mod view;

pub use view::BurgerMenuGroupView;
mod style;

use components::burger_download_item::BurgerDownloadItem;
use components::burger_help_item::BurgerHelpItem;
use components::burger_preview_item::BurgerPreviewItem;
use components::burger_redo_item::BurgerRedoItem;
use components::burger_resolve_item::BurgerResolveItem;
use components::burger_system_hotkeys_item::BurgerSystemHotkeysItem;
use components::burger_templates_item::BurgerTemplatesItem;
use components::burger_undo_item::BurgerUndoItem;
use components::burger_upload_item::BurgerUploadItem;
use dioxus::prelude::*;
use model::BurgerMenuGroupModel;
use style::CLASS;
use tw_macro::assert_component;

/// The scrolling list of file actions inside the drawer: a `role="menu"` container that names
/// each bespoke action row in order. The non-dialog rows (undo, redo, resolve) close the drawer
/// on click, so they receive its close handler; the dialog rows own their own dialogs and leave
/// the drawer open.
#[component]
pub fn BurgerMenuGroup(props: BurgerMenuGroupModel) -> Element {
    let on_close = props.on_close;
    rsx! {
        div {
            class: CLASS,
            role: "menu",
            aria_label: "File actions",
            BurgerUndoItem {
                on_close,
            }
            BurgerRedoItem {
                on_close,
            }
            BurgerUploadItem {
            


            }
            BurgerTemplatesItem {
            


            }
            BurgerSystemHotkeysItem {
            


            }
            BurgerResolveItem {
                on_close,
            }
            BurgerPreviewItem {
            


            }
            BurgerDownloadItem {
            


            }
            BurgerHelpItem {
            


            }
        }
    }
}

assert_component!(BurgerMenuGroup);
