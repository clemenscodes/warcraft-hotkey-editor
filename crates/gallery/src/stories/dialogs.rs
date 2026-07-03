use dioxus::prelude::*;
use gallery::Story;
use hotkey_editor::components::dialogs::dialog::components::dialog_header::DialogHeader;
use hotkey_editor::components::dialogs::help_dialog::HelpDialog;
use hotkey_editor::components::dialogs::info_dialogs::download_info_dialog::DownloadInfoDialog;
use hotkey_editor::services::customkeys::upload_status::UploadStatus;

use hotkey_editor::components::dialogs::key_picker::{
    KeyPicker, KeyPickerCell, KeyPickerCellState,
};

use hotkey_editor::components::dialogs::layout_editor::LayoutEditor;
use hotkey_editor::components::dialogs::preview_dialog::PreviewDialog;
use hotkey_editor::components::dialogs::templates_dialog::TemplatesDialog;
use hotkey_editor::components::dialogs::templates_dialog::components::template_gallery::components::template_card::TemplateCard;
use hotkey_editor::components::dialogs::info_dialogs::upload_info_dialog::UploadInfoDialog;
use hotkey_editor::components::grid_editors::grid_editor::components::headed_grid::HeadedGrid;
use hotkey_editor::components::grid_editors::grid_editor::components::headed_grid::components::grid::GridProps;
use hotkey_editor::components::grid_editors::grid_editor::components::grid_editor_tile::{
    EditorTileKind, GridEditorTileProps,
};
use super::keys_mount::CustomKeysMount;
use hotkey_editor::components::shell::toasts::ToastMount;

use warcraft_keybinds::{
    COMMAND_GRID_TILE_COUNT, CustomKeys, GridCoordinate, HotkeyToken, RenderedTile,
    ResolvedTemplate,
};

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
        Story::new("Dialogs", "KeyPicker", "Closed", key_picker_closed),
        Story::new("Dialogs", "KeyPicker", "Open, all states", key_picker_open),
        Story::new("Dialogs", "HeadedGrid", "Command card", headed_grid_command),
        Story::new(
            "Dialogs",
            "HeadedGrid",
            "Research menu",
            headed_grid_research,
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
    let loaded_keys = use_signal(|| Some(CustomKeys::from_text("")));
    let preview_open = use_signal(|| true);
    rsx! {
        PreviewDialog { loaded_keys, preview_open }
    }
}

fn layout_editor_default() -> Element {
    let grid_layout = use_signal(fixtures::sample_grid_layout);
    let editing_layout_cell = use_signal(|| None::<GridCoordinate>);
    let dragging_layout_cell = use_signal(|| None::<GridCoordinate>);
    let loaded_keys = use_signal(|| Some(fixtures::sample_keys()));
    let layout_dialog_open = use_signal(|| true);
    let update_hotkeys_on_move = use_signal(|| true);
    rsx! {
        ToastMount {
            CustomKeysMount {
                loaded_keys,
                LayoutEditor {
                    grid_layout,
                    editing_layout_cell,
                    dragging_layout_cell,
                    loaded_keys,
                    open: layout_dialog_open,
                    update_hotkeys_on_move,
                }
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
            TemplatesDialog {
                loaded_keys,
                upload_status,
                open: templates_dialog_open,
            }
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
            name: "Default".to_string(),
            description: "Stock Warcraft III hotkeys".to_string(),
            resolved,
            on_apply: move |_| {},
        }
    }
}

fn key_picker_closed() -> Element {
    let title = "Pick a hotkey".to_string();
    let rows: Vec<Vec<KeyPickerCell>> = Vec::new();
    let open = false;
    rsx! {
        KeyPicker {
            title,
            rows,
            open,
            on_pick: move |_| {},
            on_close: move |_| {},
        }
    }
}

fn key_picker_open() -> Element {
    let title = "Pick a hotkey".to_string();
    let q_token = HotkeyToken::try_from('Q').expect("letter");
    let q_state = KeyPickerCellState::Available;
    let q_cell = KeyPickerCell::new(q_token, q_state);
    let w_token = HotkeyToken::try_from('W').expect("letter");
    let w_state = KeyPickerCellState::Current;
    let w_cell = KeyPickerCell::new(w_token, w_state);
    let e_token = HotkeyToken::try_from('E').expect("letter");
    let conflict_name = "Some Other Ability".to_string();
    let e_state = KeyPickerCellState::Conflict {
        display_name: conflict_name,
    };
    let e_cell = KeyPickerCell::new(e_token, e_state);
    let first_row = vec![q_cell, w_cell, e_cell];
    let rows = vec![first_row];
    let open = true;
    rsx! {
        KeyPicker {
            title,
            rows,
            open,
            on_pick: move |_| {},
            on_close: move |_| {},
        }
    }
}

fn headed_grid_command() -> Element {
    let resolved_templates = use_hook(ResolvedTemplate::resolve_all);
    let resolved = resolved_templates
        .first()
        .expect("at least one bundled template")
        .clone();
    let tiles = headed_grid_tiles(resolved.command_tiles());
    let kind = EditorTileKind;
    let grid = GridProps { kind, tiles };
    rsx! {
        HeadedGrid { heading: "Command card", grid }
    }
}

fn headed_grid_research() -> Element {
    let resolved_templates = use_hook(ResolvedTemplate::resolve_all);
    let resolved = resolved_templates
        .first()
        .expect("at least one bundled template")
        .clone();
    let tiles = headed_grid_tiles(resolved.research_tiles());
    let kind = EditorTileKind;
    let grid = GridProps { kind, tiles };
    rsx! {
        HeadedGrid { heading: "Research menu", grid }
    }
}

fn headed_grid_tiles(source: &[RenderedTile]) -> [GridEditorTileProps; COMMAND_GRID_TILE_COUNT] {
    source
        .iter()
        .map(GridEditorTileProps::from)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap_or_else(|_| panic!("command grid is always {COMMAND_GRID_TILE_COUNT} tiles"))
}
