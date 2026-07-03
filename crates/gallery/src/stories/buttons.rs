use crate::stories::fixtures;
use dioxus::prelude::*;
use gallery::Story;
use hotkey_editor::components::shell::header::components::header_actions::components::header_toolbar::components::export_button_host::components::export_button::ExportButton;
use hotkey_editor::components::shell::header::components::header_layout_slot::components::grid_layout_button::GridLayoutButton;
use hotkey_editor::components::shell::header::components::header_actions::components::header_toolbar::components::help_button::HelpButton;
use hotkey_editor::components::shell::header::components::header_actions::components::header_toolbar::components::preview_button::PreviewButton;
use hotkey_editor::components::shell::header::components::header_actions::components::header_toolbar::components::redo_button::RedoButton;
use hotkey_editor::components::shell::header::components::header_actions::components::header_toolbar::components::resolve_button_host::components::resolve_button::ResolveButton;
use hotkey_editor::components::shell::header::components::header_actions::components::header_toolbar::components::system_hotkeys_button::SystemHotkeysButton;
use hotkey_editor::components::shell::header::components::header_actions::components::header_toolbar::components::templates_button::TemplatesButton;
use hotkey_editor::components::shell::header::components::header_actions::components::header_toolbar::components::undo_button::UndoButton;
use hotkey_editor::components::shell::header::components::header_actions::components::header_toolbar::components::upload_button::UploadButton;
use hotkey_editor::components::shell::header::components::header_actions::components::header_toolbar::components::shared::toolbar_button::ToolbarButton;
use hotkey_editor::components::shell::header::components::header_actions::components::collisions_button::CollisionsButton;
use hotkey_editor::components::shell::toasts::ToastMount;
use hotkey_editor::services::customkeys::upload_status::UploadStatus;
use hotkey_editor::services::navigation::app_view::AppView;
use hotkey_editor::services::navigation::view_navigation::ViewNavigationContext;
use hotkey_editor::services::overlay_state::OverlayState;
use hotkey_editor::services::undo::UndoHistory;
use warcraft_api::Race;
use warcraft_database::UnitMode;

/// Provides the app-wide overlay open state a toolbar/burger button reads from
/// context, so a button can be shown in isolation.
fn provide_overlay_state() {
    let overlay = OverlayState {
        preview_open: use_signal(|| false),
        system_hotkeys_open: use_signal(|| false),
        help_open: use_signal(|| false),
        layout_dialog_open: use_signal(|| false),
        templates_dialog_open: use_signal(|| false),
    };
    use_context_provider(|| overlay);
}

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
    provide_overlay_state();
    rsx! {
        TemplatesButton {}
    }
}

fn system_hotkeys_button() -> Element {
    provide_overlay_state();
    rsx! {
        SystemHotkeysButton {}
    }
}

fn help_button() -> Element {
    provide_overlay_state();
    rsx! {
        HelpButton {}
    }
}

fn grid_layout_button() -> Element {
    provide_overlay_state();
    rsx! {
        GridLayoutButton {}
    }
}

fn toolbar_button() -> Element {
    let noop = move |_| {};
    rsx! {
        ToolbarButton { icon: DEMO_ICON, aria_label: "Example action", onclick: noop }
    }
}

fn preview_button() -> Element {
    provide_overlay_state();
    rsx! {
        PreviewButton {}
    }
}

fn export_button() -> Element {
    let info_open = use_signal(|| false);
    rsx! {
        ExportButton {
            visible: true,
            info_open,
            on_open: move |_| {},
            on_confirm: move |_| {},
        }
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
    let disabled = false;
    rsx! {
        ResolveButton {
            disabled,
            onclick: move |_| {},
        }
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
    use_context_provider(|| navigation);
    rsx! {
        CollisionsButton { loaded_keys, grid_layout }
    }
}
