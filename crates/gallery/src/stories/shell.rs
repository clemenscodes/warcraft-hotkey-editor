use dioxus::prelude::*;
use gallery::Story;
use hotkey_editor::{
    AppView, BurgerMenu, CollisionsButton, DragFollower, DragFollowerOverlay, DragFollowerVisual,
    EditingCell, Footer, Header, HeaderBrand, HeaderToolbar, ToastMount, TooltipMount, UndoHistory,
    UploadStatus, ViewNavigationContext,
};
use warcraft_api::Race;
use warcraft_database::UnitMode;
use warcraft_keybinds::CustomKeys;

use crate::stories::fixtures;

pub fn stories() -> Vec<Story> {
    vec![
        Story::new("Shell", "Footer", footer_default),
        Story::new("Shell", "TooltipMount", tooltip_mount_default),
        Story::new("Shell", "ToastMount — with child", toast_mount_with_child),
        Story::new("Shell", "HeaderBrand", header_brand_default),
        Story::new(
            "Shell",
            "CollisionsButton — no file",
            collisions_button_no_file,
        ),
        Story::new(
            "Shell",
            "CollisionsButton — file loaded",
            collisions_button_loaded,
        ),
        Story::new("Shell", "BurgerMenu", burger_menu_default),
        Story::new("Shell", "HeaderToolbar", header_toolbar_default),
        Story::new("Shell", "Header", header_default),
        Story::new(
            "Shell",
            "DragFollowerOverlay — with ability",
            drag_follower_overlay_with_ability,
        ),
        Story::new(
            "Shell",
            "DragFollowerOverlay — empty",
            drag_follower_overlay_empty,
        ),
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
    rsx! {
        HeaderBrand { navigation }
    }
}

fn collisions_button_no_file() -> Element {
    let loaded_keys = use_signal(|| None::<CustomKeys>);
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

fn collisions_button_loaded() -> Element {
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

fn burger_menu_default() -> Element {
    let loaded_keys = use_signal(|| Some(fixtures::sample_keys()));
    let grid_layout = use_signal(fixtures::sample_grid_layout);
    let preview_open = use_signal(|| false);
    let layout_dialog_open = use_signal(|| false);
    let templates_dialog_open = use_signal(|| false);
    let system_hotkeys_open = use_signal(|| false);
    let help_open = use_signal(|| false);
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
    rsx! {
        BurgerMenu {
            loaded_keys,
            preview_open,
            layout_dialog_open,
            templates_dialog_open,
            system_hotkeys_open,
            help_open,
            navigation,
        }
    }
}

fn header_toolbar_default() -> Element {
    let loaded_keys = use_signal(|| Some(fixtures::sample_keys()));
    let grid_layout = use_signal(fixtures::sample_grid_layout);
    let upload_status = use_signal(|| UploadStatus::Idle);
    let preview_open = use_signal(|| false);
    let templates_dialog_open = use_signal(|| false);
    let system_hotkeys_open = use_signal(|| false);
    let help_open = use_signal(|| false);
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
    rsx! {
        ToastMount {
            HeaderToolbar {
                loaded_keys,
                upload_status,
                preview_open,
                templates_dialog_open,
                system_hotkeys_open,
                help_open,
                navigation,
            }
        }
    }
}

fn header_default() -> Element {
    let loaded_keys = use_signal(|| Some(fixtures::sample_keys()));
    let grid_layout = use_signal(fixtures::sample_grid_layout);
    let upload_status = use_signal(|| UploadStatus::Idle);
    let preview_open = use_signal(|| false);
    let editing_layout_cell = use_signal(|| None::<EditingCell>);
    let dragging_layout_cell = use_signal(|| None::<EditingCell>);
    let system_hotkeys_open = use_signal(|| false);
    let help_open = use_signal(|| false);
    let current_view = use_signal(|| AppView::Editor);
    let active_race = use_signal(|| Race::Human);
    let unit_mode = use_signal(|| UnitMode::Melee);
    let selected_unit_id = use_signal(|| None::<String>);
    let search_query = use_signal(String::new);
    let undo = UndoHistory::use_history(loaded_keys, grid_layout);
    use_context_provider(|| undo);
    rsx! {
        ToastMount {
            Header {
                loaded_keys,
                upload_status,
                preview_open,
                grid_layout,
                editing_layout_cell,
                dragging_layout_cell,
                system_hotkeys_open,
                help_open,
                current_view,
                active_race,
                unit_mode,
                selected_unit_id,
                search_query,
            }
        }
    }
}

fn drag_follower_overlay_with_ability() -> Element {
    let visual = DragFollowerVisual::new(
        None,
        "Sample Ability".to_string(),
        Some("Q".to_string()),
        false,
        true,
    );
    let follower = DragFollower::new(visual, 0.0, 0.0, 80.0, 80.0, 64.0, 64.0);
    let drag_follower = use_signal(|| Some(follower));
    let active_race = use_signal(|| Race::Human);
    rsx! {
        div { style: "position: relative; width: 200px; height: 200px;",
            DragFollowerOverlay { drag_follower, active_race }
        }
    }
}

fn drag_follower_overlay_empty() -> Element {
    let drag_follower = use_signal(|| None::<DragFollower>);
    let active_race = use_signal(|| Race::Human);
    rsx! {
        div { style: "position: relative; width: 200px; height: 200px;",
            DragFollowerOverlay { drag_follower, active_race }
        }
    }
}
