mod hooks;
mod route_sync;
mod style;

use crate::app::document_head::DocumentHead;
use crate::app::route::Route;
use crate::assert_component;
use crate::components::dialogs::help_dialog::HelpDialog;
use crate::components::dialogs::layout_editor::LayoutEditor;
use crate::components::dialogs::preview_dialog::PreviewDialog;
use crate::components::dialogs::system_hotkeys_dialog::SystemHotkeysDialog;
use crate::components::dialogs::templates_dialog::TemplatesDialog;
use crate::components::shell::footer::Footer;
use crate::components::shell::header::Header;
use crate::components::shell::toasts::ToastMount;
use dioxus::prelude::*;
use hooks::{ShellModel, use_shell};

assert_component!(Shell);

/// The persistent application frame: the header, the routed page (via `Outlet`), the
/// footer, and the overlay dialogs, wrapped in the toast provider and the app-root
/// element that owns the page's background, typography, and scrollbar.
///
/// This is the Dioxus **layout** shared by the three page routes, so it stays mounted
/// while the `Outlet` swaps the active page — which is what lets every signal
/// [`use_shell`] owns (the loaded keys, grid layout, editor selection) survive
/// navigation between the editor, collisions, and resolve pages. It replaces the old
/// one-page `Workbench` god-component: its body is a flat list of children, every
/// piece of app-wide state reaching the pages and header through context rather than
/// props.
#[component]
pub fn Shell() -> Element {
    let ShellModel {
        app_class,
        handle_keydown,
        loaded_keys,
        grid_layout,
        update_hotkeys_on_move,
        upload_status,
        preview_open,
        system_hotkeys_open,
        help_open,
        layout_dialog_open,
        templates_dialog_open,
        editing_layout_cell,
        dragging_layout_cell,
    } = use_shell();
    rsx! {
        DocumentHead {}
        ToastMount {
            div {
                class: app_class,
                onkeydown: handle_keydown,
                Header {}
                Outlet::<Route> {}
                Footer {}
                PreviewDialog { loaded_keys, preview_open }
                SystemHotkeysDialog { loaded_keys, system_hotkeys_open }
                HelpDialog { help_open }
                TemplatesDialog {
                    loaded_keys,
                    upload_status,
                    open: templates_dialog_open,
                }
                LayoutEditor {
                    grid_layout,
                    editing_layout_cell,
                    dragging_layout_cell,
                    update_hotkeys_on_move,
                    open: layout_dialog_open,
                }
            }
        }
    }
}
