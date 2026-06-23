use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

use crate::components::actions::export_buttons::ExportButtons;
use crate::components::actions::resolve_button::ResolveButton;
use crate::components::actions::undo_redo_buttons::UndoRedoButtons;
use crate::components::actions::upload_button::UploadButton;
use crate::components::shared::icons::{ICON_COG, ICON_HELP, ICON_TEMPLATES};
use crate::services::customkeys::upload_status::UploadStatus;
use crate::services::navigation::view_navigation::ViewNavigationContext;

#[derive(Props, Clone, PartialEq)]
pub(crate) struct HeaderToolbarProps {
    pub(crate) loaded_keys: Signal<Option<CustomKeys>>,
    pub(crate) upload_status: Signal<UploadStatus>,
    pub(crate) preview_open: Signal<bool>,
    pub(crate) templates_dialog_open: Signal<bool>,
    pub(crate) system_hotkeys_open: Signal<bool>,
    pub(crate) help_open: Signal<bool>,
    pub(crate) navigation: ViewNavigationContext,
}

#[component]
pub(crate) fn HeaderToolbar(props: HeaderToolbarProps) -> Element {
    let loaded_keys = props.loaded_keys;
    let upload_status = props.upload_status;
    let preview_open = props.preview_open;
    let navigation = props.navigation;
    let mut templates_dialog_open = props.templates_dialog_open;
    let mut system_hotkeys_open = props.system_hotkeys_open;
    let mut help_open = props.help_open;
    let open_help = move |_| help_open.set(true);
    let toggle_templates = move |_| {
        let next = !*templates_dialog_open.read();
        templates_dialog_open.set(next);
    };
    let toggle_system_hotkeys = move |_| {
        let next = !*system_hotkeys_open.read();
        system_hotkeys_open.set(next);
    };
    rsx! {
        div {
            class: "hidden flex-row items-center justify-end \
                    [gap:calc(0.65rem_*_var(--hdr-scale))] min-w-0 \
                    min-[1500px]:flex",
            role: "toolbar",
            aria_label: "File actions",
            UndoRedoButtons {}
            UploadButton { loaded_keys, upload_status }
            button {
                class: super::TOOLBAR_BTN_CLASS,
                r#type: "button",
                aria_label: "Browse layout templates",
                aria_haspopup: "dialog",
                aria_expanded: "{templates_dialog_open()}",
                onclick: toggle_templates,
                span {
                    class: super::TOOLBAR_ICON_CLASS,
                    aria_hidden: "true",
                    dangerous_inner_html: ICON_TEMPLATES,
                }
            }
            button {
                class: super::TOOLBAR_BTN_CLASS,
                r#type: "button",
                aria_label: "General hotkeys",
                aria_haspopup: "dialog",
                aria_expanded: "{system_hotkeys_open()}",
                onclick: toggle_system_hotkeys,
                span {
                    class: super::TOOLBAR_ICON_CLASS,
                    aria_hidden: "true",
                    dangerous_inner_html: ICON_COG,
                }
            }
            ResolveButton { loaded_keys, navigation }
            ExportButtons { loaded_keys, preview_open }
            button {
                class: super::TOOLBAR_BTN_CLASS,
                r#type: "button",
                aria_label: "How to use this editor",
                aria_haspopup: "dialog",
                aria_expanded: "{help_open()}",
                onclick: open_help,
                span {
                    class: super::TOOLBAR_ICON_CLASS,
                    aria_hidden: "true",
                    dangerous_inner_html: ICON_HELP,
                }
            }
        }
    }
}
