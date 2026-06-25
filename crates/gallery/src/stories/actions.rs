use dioxus::prelude::*;
use gallery::Story;
use hotkey_editor::{
    AppView, ExportButtons, ResolveButton, ToastMount, UndoHistory, UndoRedoButtons, UploadButton,
    UploadStatus, ViewNavigationContext,
};
use warcraft_api::Race;
use warcraft_database::UnitMode;
use warcraft_keybinds::CustomKeys;

use crate::stories::fixtures;

pub fn stories() -> Vec<Story> {
    vec![
        Story::new("Actions", "Export buttons — loaded", export_buttons_loaded),
        Story::new(
            "Actions",
            "Export buttons — no file",
            export_buttons_no_file,
        ),
        Story::new(
            "Actions",
            "Resolve button — disabled (no file)",
            resolve_button_disabled,
        ),
        Story::new(
            "Actions",
            "Resolve button — enabled",
            resolve_button_enabled,
        ),
        Story::new("Actions", "UndoRedoButtons", undo_redo_buttons_default),
        Story::new("Actions", "UploadButton — no file", upload_button_no_file),
        Story::new(
            "Actions",
            "UploadButton — file loaded",
            upload_button_loaded,
        ),
    ]
}

fn export_buttons_loaded() -> Element {
    let loaded_keys = use_signal(|| Some(CustomKeys::from("").normalize()));
    let preview_open = use_signal(|| false);
    rsx! {
        ExportButtons { loaded_keys, preview_open }
    }
}

fn export_buttons_no_file() -> Element {
    let loaded_keys = use_signal(|| None::<CustomKeys>);
    let preview_open = use_signal(|| false);
    rsx! {
        ExportButtons { loaded_keys, preview_open }
    }
}

fn resolve_button_disabled() -> Element {
    let loaded_keys = use_signal(|| None::<CustomKeys>);
    let current_view = use_signal(|| AppView::Editor);
    let active_race = use_signal(|| Race::Human);
    let unit_mode = use_signal(|| UnitMode::Melee);
    let selected_unit_id = use_signal(|| None::<String>);
    let search_query = use_signal(String::new);
    let navigation = ViewNavigationContext {
        current_view,
        active_race,
        unit_mode,
        selected_unit_id,
        search_query,
    };
    rsx! {
        ResolveButton { loaded_keys, navigation }
    }
}

fn resolve_button_enabled() -> Element {
    let loaded_keys = use_signal(|| Some(CustomKeys::from("").normalize()));
    let current_view = use_signal(|| AppView::Editor);
    let active_race = use_signal(|| Race::Human);
    let unit_mode = use_signal(|| UnitMode::Melee);
    let selected_unit_id = use_signal(|| None::<String>);
    let search_query = use_signal(String::new);
    let navigation = ViewNavigationContext {
        current_view,
        active_race,
        unit_mode,
        selected_unit_id,
        search_query,
    };
    rsx! {
        ResolveButton { loaded_keys, navigation }
    }
}

fn undo_redo_buttons_default() -> Element {
    let keys = use_signal(|| Some(fixtures::sample_keys()));
    let layout = use_signal(fixtures::sample_grid_layout);
    let undo = UndoHistory::use_history(keys, layout);
    use_context_provider(|| undo);
    rsx! {
        UndoRedoButtons {}
    }
}

fn upload_button_no_file() -> Element {
    let loaded_keys = use_signal(|| None::<CustomKeys>);
    let upload_status = use_signal(|| UploadStatus::Idle);
    rsx! {
        ToastMount {
            UploadButton { loaded_keys, upload_status }
        }
    }
}

fn upload_button_loaded() -> Element {
    let loaded_keys = use_signal(|| Some(fixtures::sample_keys()));
    let upload_status = use_signal(|| UploadStatus::Loaded {
        binding_count: 42,
        command_count: 12,
    });
    rsx! {
        ToastMount {
            UploadButton { loaded_keys, upload_status }
        }
    }
}
