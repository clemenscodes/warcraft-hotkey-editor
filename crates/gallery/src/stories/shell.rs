use crate::stories::fixtures;
use dioxus::prelude::*;
use gallery::Story;
use hotkey_editor::components::app::components::shell::components::footer::Footer;
use hotkey_editor::components::app::components::shell::components::header::Header;
use hotkey_editor::components::app::components::shell::components::header::components::brand_host::BrandHost;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::ToolbarActions;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::BurgerMenu;
use hotkey_editor::components::app::components::shell::components::toasts::Toasts;
use hotkey_editor::services::customkeys::service::CustomKeysService;
use hotkey_editor::services::customkeys::upload_status::UploadStatus;
use hotkey_editor::services::navigation::app_view::AppView;
use hotkey_editor::services::navigation::view_navigation::ViewNavigationContext;
use hotkey_editor::services::overlay_state::OverlayState;
use hotkey_editor::services::undo::UndoHistory;
use warcraft_api::Race;
use warcraft_database::UnitMode;

/// Provides the app-wide overlay open state the header, toolbar, and burger read
/// from context, so those components can be shown in isolation.
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

/// Provides the navigation context the header's brand, collisions, and burger read
/// from context, so those components can be shown in isolation.
fn provide_navigation() {
    let current_view = use_signal(|| AppView::Editor);
    let active_race = use_signal(|| Race::Human);
    let unit_mode = use_signal(|| UnitMode::Melee);
    let selected_unit_id = use_signal(|| None::<String>);
    let search_query = use_signal(String::new);
    let navigation = ViewNavigationContext::new(
        current_view,
        active_race,
        unit_mode,
        selected_unit_id,
        search_query,
    );
    use_context_provider(|| navigation);
}

pub fn stories() -> Vec<Story> {
    vec![
        Story::single("Shell", "Footer", footer_default),
        Story::single("Shell", "Toasts", toast_mount_with_child),
        Story::single("Shell", "BrandHost", brand_default),
        Story::single("Shell", "BurgerMenu", burger_menu_default),
        Story::single("Shell", "ToolbarActions", toolbar_actions_default),
        Story::single("Shell", "Header", header_default),
    ]
}

fn footer_default() -> Element {
    rsx! {
        Footer {}
    }
}

fn toast_mount_with_child() -> Element {
    rsx! {
        Toasts {
            div { "Toast provider mounted" }
        }
    }
}

fn brand_default() -> Element {
    provide_navigation();
    rsx! {
        BrandHost {}
    }
}

fn burger_menu_default() -> Element {
    let loaded_keys = use_signal(|| Some(fixtures::sample_keys()));
    let grid_layout = use_signal(fixtures::sample_grid_layout);
    let undo = UndoHistory::use_history(loaded_keys, grid_layout);
    let custom_keys_service = CustomKeysService::new(loaded_keys);
    use_context_provider(|| custom_keys_service);
    use_context_provider(|| undo);
    provide_navigation();
    provide_overlay_state();
    rsx! {
        BurgerMenu {}
    }
}

fn toolbar_actions_default() -> Element {
    let loaded_keys = use_signal(|| Some(fixtures::sample_keys()));
    let grid_layout = use_signal(fixtures::sample_grid_layout);
    let upload_status = use_signal(|| UploadStatus::Idle);
    let undo = UndoHistory::use_history(loaded_keys, grid_layout);
    let custom_keys_service = CustomKeysService::new(loaded_keys);
    use_context_provider(|| custom_keys_service);
    use_context_provider(|| upload_status);
    use_context_provider(|| undo);
    provide_navigation();
    provide_overlay_state();
    rsx! {
        Toasts {
            ToolbarActions {}
        }
    }
}

fn header_default() -> Element {
    let loaded_keys = use_signal(|| Some(fixtures::sample_keys()));
    let grid_layout = use_signal(fixtures::sample_grid_layout);
    let upload_status = use_signal(|| UploadStatus::Idle);
    let undo = UndoHistory::use_history(loaded_keys, grid_layout);
    let custom_keys_service = CustomKeysService::new(loaded_keys);
    use_context_provider(|| custom_keys_service);
    use_context_provider(|| grid_layout);
    use_context_provider(|| upload_status);
    use_context_provider(|| undo);
    provide_navigation();
    provide_overlay_state();
    rsx! {
        Toasts {
            Header {}
        }
    }
}
