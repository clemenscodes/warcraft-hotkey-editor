pub mod components;
pub mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::HelpDialog;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::download_info_dialog::DownloadInfoDialog;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::upload_info_dialog::UploadInfoDialog;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::preview_dialog::PreviewDialog;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::SystemHotkeysDialog;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::templates_dialog::TemplatesDialog;
use crate::services::overlay_state::context::use_overlay_state;
use components::burger_menu::BurgerMenu;
use components::inline_actions::InlineActions;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// The adaptive file-action controls: the inline button row at laptop width and up, the
/// burger drawer on narrower screens, and every file-action dialog mounted once here. It
/// resolves the nine file actions once and provides them to the subtree, so the inline
/// buttons and the burger rows read one shared set; each dialog renders `if open` off its
/// own overlay signal from this always-present container, so no dialog is tied to a trigger.
#[component]
pub fn ToolbarActions() -> Element {
    let overlay = use_overlay_state();
    let mut upload_info_open = overlay.upload_info_open();
    let mut download_info_open = overlay.download_info_open();
    rsx! {
        div {
            class: CLASS,
            InlineActions {}
            BurgerMenu {}
            HelpDialog {}
            TemplatesDialog {}
            SystemHotkeysDialog {}
            PreviewDialog {}
            UploadInfoDialog {
                open: *upload_info_open.read(),
                on_open_change: Callback::new(move |value: bool| upload_info_open.set(value)),
            }
            DownloadInfoDialog {
                open: *download_info_open.read(),
                on_open_change: Callback::new(move |value: bool| download_info_open.set(value)),
            }
        }
    }
}

assert_component!(ToolbarActions);
