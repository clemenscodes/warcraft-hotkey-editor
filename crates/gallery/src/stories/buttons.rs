use dioxus::prelude::*;
use gallery::Story;
use hotkey_editor::{
    AppView, CollisionsButton, ExportButton, GridLayoutButton, HelpButton, PreviewButton,
    RedoButton, ResolveButton, SystemHotkeysButton, TemplatesButton, ToastMount, ToolbarButton,
    UndoButton, UndoHistory, UploadButton, UploadStatus, ViewNavigationContext,
};
use warcraft_api::Race;
use warcraft_database::UnitMode;
use warcraft_keybinds::CustomKeys;

use crate::stories::fixtures;

/// Placeholder glyph for the base button showcase: a plain rounded square so the
/// story shows the shared styling without implying a specific action.
const DEMO_ICON: &str = "<svg viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" \
    stroke-width=\"2\"><rect x=\"4\" y=\"4\" width=\"16\" height=\"16\" rx=\"3\" /></svg>";

pub fn stories() -> Vec<Story> {
    vec![
        Story::single("Buttons", "ToolbarButton", toolbar_button),
        Story::single("Buttons", "UndoButton", undo_button),
        Story::single("Buttons", "RedoButton", redo_button),
        Story::single("Buttons", "UploadButton", upload_button),
        Story::single("Buttons", "TemplatesButton", templates_button),
        Story::single("Buttons", "SystemHotkeysButton", system_hotkeys_button),
        Story::single("Buttons", "ResolveButton", resolve_button),
        Story::single("Buttons", "PreviewButton", preview_button),
        Story::single("Buttons", "ExportButton", export_button),
        Story::single("Buttons", "HelpButton", help_button),
        Story::single("Buttons", "GridLayoutButton", grid_layout_button),
        Story::single("Buttons", "CollisionsButton", collisions_button),
    ]
}

fn templates_button() -> Element {
    let templates_dialog_open = use_signal(|| false);
    rsx! {
        TemplatesButton { templates_dialog_open }
    }
}

fn system_hotkeys_button() -> Element {
    let system_hotkeys_open = use_signal(|| false);
    rsx! {
        SystemHotkeysButton { system_hotkeys_open }
    }
}

fn help_button() -> Element {
    let help_open = use_signal(|| false);
    rsx! {
        HelpButton { help_open }
    }
}

fn grid_layout_button() -> Element {
    let layout_dialog_open = use_signal(|| false);
    rsx! {
        GridLayoutButton { layout_dialog_open }
    }
}

fn toolbar_button() -> Element {
    let noop = move |_| {};
    rsx! {
        ToolbarButton { icon: DEMO_ICON, aria_label: "Example action", onclick: noop }
    }
}

fn preview_button() -> Element {
    let preview_open = use_signal(|| false);
    rsx! {
        PreviewButton { preview_open }
    }
}

fn export_button() -> Element {
    let loaded_keys = use_signal(|| Some(CustomKeys::from("").normalize()));
    rsx! {
        ExportButton { loaded_keys }
    }
}

fn undo_button() -> Element {
    let keys = use_signal(|| Some(fixtures::sample_keys()));
    let layout = use_signal(fixtures::sample_grid_layout);
    let undo = UndoHistory::use_history(keys, layout);
    use_context_provider(|| undo);
    rsx! {
        UndoButton {}
    }
}

fn redo_button() -> Element {
    let keys = use_signal(|| Some(fixtures::sample_keys()));
    let layout = use_signal(fixtures::sample_grid_layout);
    let undo = UndoHistory::use_history(keys, layout);
    use_context_provider(|| undo);
    rsx! {
        RedoButton {}
    }
}

fn upload_button() -> Element {
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

fn resolve_button() -> Element {
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

fn collisions_button() -> Element {
    let loaded_keys = use_signal(|| Some(fixtures::sample_keys()));
    let grid_layout = use_signal(fixtures::sample_grid_layout);
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
        CollisionsButton { loaded_keys, grid_layout, navigation }
    }
}
