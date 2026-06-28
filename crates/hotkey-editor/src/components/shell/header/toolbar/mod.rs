use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

use crate::components::actions::export_button::ExportButton;
use crate::components::actions::help_button::HelpButton;
use crate::components::actions::preview_button::PreviewButton;
use crate::components::actions::redo_button::RedoButton;
use crate::components::actions::resolve_button::ResolveButton;
use crate::components::actions::system_hotkeys_button::SystemHotkeysButton;
use crate::components::actions::templates_button::TemplatesButton;
use crate::components::actions::undo_button::UndoButton;
use crate::components::actions::upload_button::UploadButton;
use crate::services::customkeys::upload_status::UploadStatus;
use crate::services::navigation::view_navigation::ViewNavigationContext;

#[derive(Props, Clone, PartialEq)]
pub struct HeaderToolbarProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub upload_status: Signal<UploadStatus>,
    pub preview_open: Signal<bool>,
    pub templates_dialog_open: Signal<bool>,
    pub system_hotkeys_open: Signal<bool>,
    pub help_open: Signal<bool>,
    pub navigation: ViewNavigationContext,
}

#[component]
pub fn HeaderToolbar(props: HeaderToolbarProps) -> Element {
    let loaded_keys = props.loaded_keys;
    let upload_status = props.upload_status;
    let preview_open = props.preview_open;
    let navigation = props.navigation;
    let templates_dialog_open = props.templates_dialog_open;
    let system_hotkeys_open = props.system_hotkeys_open;
    let help_open = props.help_open;
    rsx! {
        div {
            class: "hidden flex-row items-center justify-end \
                    [gap:calc(0.65rem_*_var(--hdr-scale))] min-w-0 \
                    min-[1500px]:flex",
            role: "toolbar",
            aria_label: "File actions",
            UndoButton {}
            RedoButton {}
            UploadButton { loaded_keys, upload_status }
            TemplatesButton { templates_dialog_open }
            SystemHotkeysButton { system_hotkeys_open }
            ResolveButton { loaded_keys, navigation }
            PreviewButton { preview_open }
            ExportButton { loaded_keys }
            HelpButton { help_open }
        }
    }
}
