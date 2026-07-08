use crate::stories::fixtures;
use super::toast_mount::ToastMount;
use dioxus::prelude::*;
use dioxus_gallery::Story;
use hotkey_editor::components::app::components::shell::components::header::components::grid_layout_button_host::components::grid_layout_button::GridLayoutButton;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::collisions_button_host::components::collisions_button::CollisionsButton;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::export_button::ExportButton;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::help_button::HelpButton;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::preview_button::PreviewButton;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::redo_button::RedoButton;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::resolve_button::ResolveButton;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::system_hotkeys_button::SystemHotkeysButton;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::templates_button::TemplatesButton;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::undo_button::UndoButton;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::upload_button::UploadButton;
use hotkey_editor::services::customkeys::service::CustomKeysService;
use hotkey_editor::services::customkeys::upload_status::UploadStatus;
use hotkey_editor::services::navigation::app_view::AppView;
use hotkey_editor::services::navigation::view_navigation::ViewNavigationContext;
use hotkey_editor::services::overlay_state::OverlayState;
use hotkey_editor::services::undo::UndoHistory;
use warcraft_api::Race;
use warcraft_database::UnitMode;
use warcraft_keybinds::CollisionSummary;

/// Provides the app-wide overlay open state a toolbar/burger button reads from
/// context, so a button can be shown in isolation.
fn provide_overlay_state() {
    let preview_open = use_signal(|| false);
    let system_hotkeys_open = use_signal(|| false);
    let help_open = use_signal(|| false);
    let layout_dialog_open = use_signal(|| false);
    let templates_dialog_open = use_signal(|| false);
    let overlay = OverlayState::new(
        preview_open,
        system_hotkeys_open,
        help_open,
        layout_dialog_open,
        templates_dialog_open,
    );
    use_context_provider(|| overlay);
}

/// Provides the app-wide view-navigation context a routing button reads from context,
/// so a button that navigates on click can be shown in isolation.
fn make_view_navigation() -> ViewNavigationContext {
    let current_view = use_signal(|| AppView::Editor);
    let active_race = use_signal(|| Race::Human);
    let unit_mode = use_signal(|| UnitMode::Melee);
    let selected_unit_id = use_signal(|| None::<String>);
    let search_query = use_signal(String::new);
    ViewNavigationContext::new(
        current_view,
        active_race,
        unit_mode,
        selected_unit_id,
        search_query,
    )
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
    let onclick = move |_| {};
    rsx! {
        GridLayoutButton { is_open: false, onclick }
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
    let loaded_keys = use_signal(|| Some(fixtures::sample_keys()));
    let custom_keys_service = CustomKeysService::new(loaded_keys);
    use_context_provider(|| custom_keys_service);
    rsx! {
        ExportButton {}
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
    let custom_keys_service = CustomKeysService::new(loaded_keys);
    use_context_provider(|| custom_keys_service);
    use_context_provider(|| upload_status);
    rsx! {
        ToastMount {
            UploadButton {}
        }
    }
}

fn resolve_button() -> Element {
    let loaded_keys = use_signal(|| Some(fixtures::sample_keys()));
    let custom_keys_service = CustomKeysService::new(loaded_keys);
    use_context_provider(|| custom_keys_service);
    use_context_provider(make_view_navigation);
    rsx! {
        ResolveButton {}
    }
}

fn collisions_button() -> Element {
    let keys = fixtures::sample_keys();
    let layout = fixtures::sample_grid_layout();
    let summary = CollisionSummary::compute(&keys, layout);
    rsx! {
        CollisionsButton {
            summary,
            onclick: move |_| {},
        }
    }
}
