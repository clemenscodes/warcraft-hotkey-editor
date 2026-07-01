use dioxus::prelude::*;
use gallery::Story;
use hotkey_editor::components::shell::footer::Footer;
use hotkey_editor::components::shell::header::Header;
use hotkey_editor::components::shell::header::components::burger_menu::BurgerMenu;
use hotkey_editor::components::shell::header::components::header_brand::HeaderBrand;
use hotkey_editor::components::shell::header::components::header_toolbar::HeaderToolbar;
use hotkey_editor::components::shell::toasts::ToastMount;
use hotkey_editor::components::shell::tooltips::TooltipMount;
use hotkey_editor::{AppView, OverlayState, UndoHistory, UploadStatus, ViewNavigationContext};
use warcraft_api::Race;
use warcraft_database::UnitMode;

use crate::stories::fixtures;

/// Provides the app-wide overlay open state the header, toolbar, and burger read
/// from context, so those components can be shown in isolation.
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

pub fn stories() -> Vec<Story> {
    vec![
        Story::single("Shell", "Footer", footer_default),
        Story::single("Shell", "TooltipMount", tooltip_mount_default),
        Story::single("Shell", "ToastMount", toast_mount_with_child),
        Story::single("Shell", "HeaderBrand", header_brand_default),
        Story::single("Shell", "BurgerMenu", burger_menu_default),
        Story::single("Shell", "HeaderToolbar", header_toolbar_default),
        Story::single("Shell", "Header", header_default),
    ]
}

fn footer_default() -> Element {
    rsx! {
        Footer {}
    }
}

fn tooltip_mount_default() -> Element {
    rsx! {
        TooltipMount {}
    }
}

fn toast_mount_with_child() -> Element {
    rsx! {
        ToastMount {
            div { "Toast provider mounted" }
        }
    }
}

fn header_brand_default() -> Element {
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
    let onclick = EventHandler::new(move |_event: MouseEvent| navigation.apply(AppView::Editor));
    rsx! {
        HeaderBrand { onclick }
    }
}

fn burger_menu_default() -> Element {
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
    let undo = UndoHistory::use_history(loaded_keys, grid_layout);
    use_context_provider(|| undo);
    use_context_provider(|| navigation);
    provide_overlay_state();
    rsx! {
        BurgerMenu { loaded_keys }
    }
}

fn header_toolbar_default() -> Element {
    let loaded_keys = use_signal(|| Some(fixtures::sample_keys()));
    let grid_layout = use_signal(fixtures::sample_grid_layout);
    let upload_status = use_signal(|| UploadStatus::Idle);
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
    let undo = UndoHistory::use_history(loaded_keys, grid_layout);
    use_context_provider(|| undo);
    use_context_provider(|| navigation);
    provide_overlay_state();
    rsx! {
        ToastMount {
            HeaderToolbar { loaded_keys, upload_status }
        }
    }
}

fn header_default() -> Element {
    let loaded_keys = use_signal(|| Some(fixtures::sample_keys()));
    let grid_layout = use_signal(fixtures::sample_grid_layout);
    let upload_status = use_signal(|| UploadStatus::Idle);
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
    let undo = UndoHistory::use_history(loaded_keys, grid_layout);
    use_context_provider(|| undo);
    use_context_provider(|| navigation);
    provide_overlay_state();
    rsx! {
        ToastMount {
            Header {
                loaded_keys,
                upload_status,
                grid_layout,
            }
        }
    }
}
