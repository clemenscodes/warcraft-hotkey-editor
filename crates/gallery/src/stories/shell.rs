use dioxus::prelude::*;
use gallery::Story;
use hotkey_editor::{
    AppView, BurgerMenu, EditingCell, Footer, Header, HeaderBrand, HeaderToolbar, ToastMount,
    TooltipMount, UndoHistory, UploadStatus, ViewNavigationContext,
};
use warcraft_api::Race;
use warcraft_database::UnitMode;

use crate::stories::fixtures;

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
    rsx! {
        HeaderBrand { navigation }
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
    let update_hotkeys_on_move = use_signal(|| true);
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
                update_hotkeys_on_move,
            }
        }
    }
}
