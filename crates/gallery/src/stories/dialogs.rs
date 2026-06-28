use dioxus::prelude::*;
use gallery::Story;
use hotkey_editor::{
    DialogHeader, DownloadInfoDialog, EditingCell, HelpDialog, LayoutEditor, PreviewDialog,
    ResolvedTemplate, TemplateCard, TemplateCardGrid, TemplatesDialog, ToastMount,
    UploadInfoDialog, UploadStatus,
};
use warcraft_keybinds::CustomKeys;

use crate::stories::fixtures;

pub fn stories() -> Vec<Story> {
    vec![
        Story::single("Dialogs", "DialogHeader", dialog_header_default),
        Story::single("Dialogs", "UploadInfoDialog", upload_info_dialog_open),
        Story::single("Dialogs", "DownloadInfoDialog", download_info_dialog_open),
        Story::single("Dialogs", "HelpDialog", help_dialog_open),
        Story::single("Dialogs", "PreviewDialog", preview_dialog_open),
        Story::single("Dialogs", "LayoutEditor", layout_editor_default),
        Story::single("Dialogs", "TemplatesDialog", templates_dialog_open_story),
        Story::single("Dialogs", "TemplateCard", template_card_default),
        Story::new(
            "Dialogs",
            "TemplateCardGrid",
            "Command card",
            template_card_grid_command,
        ),
        Story::new(
            "Dialogs",
            "TemplateCardGrid",
            "Research menu",
            template_card_grid_research,
        ),
    ]
}

fn dialog_header_default() -> Element {
    rsx! {
        DialogHeader { title: "Hero Abilities".to_string(), on_close: move |_| {} }
    }
}

fn upload_info_dialog_open() -> Element {
    let open = use_signal(|| true);
    rsx! {
        UploadInfoDialog { open }
    }
}

fn download_info_dialog_open() -> Element {
    let open = use_signal(|| true);
    rsx! {
        DownloadInfoDialog { open, on_confirm: move |_| {} }
    }
}

fn help_dialog_open() -> Element {
    let help_open = use_signal(|| true);
    rsx! {
        HelpDialog { help_open }
    }
}

fn preview_dialog_open() -> Element {
    let loaded_keys = use_signal(|| Some(CustomKeys::from("").normalize()));
    let preview_open = use_signal(|| true);
    rsx! {
        PreviewDialog { loaded_keys, preview_open }
    }
}

fn layout_editor_default() -> Element {
    let grid_layout = use_signal(fixtures::sample_grid_layout);
    let editing_layout_cell = use_signal(|| None::<EditingCell>);
    let dragging_layout_cell = use_signal(|| None::<EditingCell>);
    let loaded_keys = use_signal(|| Some(fixtures::sample_keys()));
    let layout_dialog_open = use_signal(|| true);
    let update_hotkeys_on_move = use_signal(|| true);
    rsx! {
        ToastMount {
            LayoutEditor {
                grid_layout,
                editing_layout_cell,
                dragging_layout_cell,
                loaded_keys,
                layout_dialog_open,
                update_hotkeys_on_move,
            }
        }
    }
}

fn templates_dialog_open_story() -> Element {
    let loaded_keys = use_signal(|| Some(fixtures::sample_keys()));
    let upload_status = use_signal(|| UploadStatus::Idle);
    let templates_dialog_open = use_signal(|| true);
    rsx! {
        ToastMount {
            TemplatesDialog { loaded_keys, upload_status, templates_dialog_open }
        }
    }
}

fn template_card_default() -> Element {
    let resolved_templates = use_hook(ResolvedTemplate::resolve_all);
    let resolved = resolved_templates
        .first()
        .expect("at least one bundled template")
        .clone();
    rsx! {
        TemplateCard {
            template_name: "Default".to_string(),
            template_description: "Stock Warcraft III hotkeys".to_string(),
            template_content: "",
            template_resolved: resolved,
            on_apply: move |_| {},
        }
    }
}

fn template_card_grid_command() -> Element {
    let resolved_templates = use_hook(ResolvedTemplate::resolve_all);
    let resolved = resolved_templates
        .first()
        .expect("at least one bundled template")
        .clone();
    rsx! {
        TemplateCardGrid {
            label: "Command card".to_string(),
            resolved,
            is_research: false,
        }
    }
}

fn template_card_grid_research() -> Element {
    let resolved_templates = use_hook(ResolvedTemplate::resolve_all);
    let resolved = resolved_templates
        .first()
        .expect("at least one bundled template")
        .clone();
    rsx! {
        TemplateCardGrid {
            label: "Research menu".to_string(),
            resolved,
            is_research: true,
        }
    }
}
