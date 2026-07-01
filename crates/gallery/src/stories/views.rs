use dioxus::prelude::*;
use hotkey_editor::components::shell::toasts::ToastMount;
use hotkey_editor::components::views::collisions_page::{
    CollisionsPage, HotkeyUnitDetail, HotkeyUnitSidebar, IslandDetail, IslandMiniGrid,
    IslandSidebar, UnitPositionDetail, UnitPositionSidebar,
};
use hotkey_editor::components::views::resolve_page::ResolvePage;
use hotkey_editor::services::navigation::app_view::{AppView, CollisionKind};
use hotkey_editor::services::navigation::view_navigation::ViewNavigationContext;
use warcraft_api::Race;
use warcraft_database::UnitMode;

use super::fixtures::{sample_grid_layout, sample_keys};
use gallery::Story;

fn make_view_navigation() -> ViewNavigationContext {
    let current_view = use_signal(|| AppView::Editor);
    let active_race = use_signal(|| Race::Human);
    let unit_mode = use_signal(|| UnitMode::Melee);
    let selected_unit_id = use_signal(|| None::<String>);
    let search_query = use_signal(String::new);
    ViewNavigationContext {
        current_view,
        active_race,
        unit_mode,
        selected_unit_id,
        search_query,
    }
}

fn collisions_page_positions() -> Element {
    let loaded_keys = use_signal(|| Some(sample_keys()));
    let grid_layout = use_signal(sample_grid_layout);
    use_context_provider(make_view_navigation);
    let selected_island = use_signal(|| None::<String>);
    let selected_hotkey_unit = use_signal(|| None::<String>);
    let selected_unit_position = use_signal(|| None::<String>);
    let kind = CollisionKind::Positions;
    rsx! {
        CollisionsPage {
            kind,
            loaded_keys,
            grid_layout,
            selected_island,
            selected_hotkey_unit,
            selected_unit_position,
        }
    }
}

fn collisions_page_hotkeys() -> Element {
    let loaded_keys = use_signal(|| Some(sample_keys()));
    let grid_layout = use_signal(sample_grid_layout);
    use_context_provider(make_view_navigation);
    let selected_island = use_signal(|| None::<String>);
    let selected_hotkey_unit = use_signal(|| None::<String>);
    let selected_unit_position = use_signal(|| None::<String>);
    let kind = CollisionKind::Hotkeys;
    rsx! {
        CollisionsPage {
            kind,
            loaded_keys,
            grid_layout,
            selected_island,
            selected_hotkey_unit,
            selected_unit_position,
        }
    }
}

fn collisions_page_unit_positions() -> Element {
    let loaded_keys = use_signal(|| Some(sample_keys()));
    let grid_layout = use_signal(sample_grid_layout);
    use_context_provider(make_view_navigation);
    let selected_island = use_signal(|| None::<String>);
    let selected_hotkey_unit = use_signal(|| None::<String>);
    let selected_unit_position = use_signal(|| None::<String>);
    let kind = CollisionKind::UnitPositions;
    rsx! {
        CollisionsPage {
            kind,
            loaded_keys,
            grid_layout,
            selected_island,
            selected_hotkey_unit,
            selected_unit_position,
        }
    }
}

fn resolve_page_story() -> Element {
    let loaded_keys = use_signal(|| Some(sample_keys()));
    use_context_provider(make_view_navigation);
    let selected_move_category = use_signal(|| None::<String>);
    rsx! {
        ToastMount {
            ResolvePage {
                loaded_keys,
                selected_move_category,
            }
        }
    }
}

fn island_mini_grid_top_left() -> Element {
    let collision_column: u8 = 0;
    let collision_row: u8 = 0;
    rsx! {
        IslandMiniGrid { collision_column, collision_row }
    }
}

fn island_mini_grid_center() -> Element {
    let collision_column: u8 = 1;
    let collision_row: u8 = 1;
    rsx! {
        IslandMiniGrid { collision_column, collision_row }
    }
}

fn island_mini_grid_bottom_right() -> Element {
    let collision_column: u8 = 3;
    let collision_row: u8 = 2;
    rsx! {
        IslandMiniGrid { collision_column, collision_row }
    }
}

fn island_sidebar_empty() -> Element {
    let islands = Vec::new();
    let selected_island = use_signal(|| None::<String>);
    rsx! {
        IslandSidebar { islands, selected_island }
    }
}

fn island_detail_empty() -> Element {
    let islands = Vec::new();
    let selected_island = use_signal(|| None::<String>);
    let view_navigation = make_view_navigation();
    rsx! {
        IslandDetail { islands, selected_island, view_navigation }
    }
}

fn hotkey_unit_sidebar_empty() -> Element {
    let units = Vec::new();
    let selected_unit = use_signal(|| None::<String>);
    rsx! {
        HotkeyUnitSidebar { units, selected_unit }
    }
}

fn hotkey_unit_detail_empty() -> Element {
    let units = Vec::new();
    let selected_unit = use_signal(|| None::<String>);
    let view_navigation = make_view_navigation();
    rsx! {
        HotkeyUnitDetail { units, selected_unit, view_navigation }
    }
}

fn unit_position_sidebar_empty() -> Element {
    let units = Vec::new();
    let selected_unit = use_signal(|| None::<String>);
    rsx! {
        UnitPositionSidebar { units, selected_unit }
    }
}

fn unit_position_detail_empty() -> Element {
    let units = Vec::new();
    let selected_unit = use_signal(|| None::<String>);
    let view_navigation = make_view_navigation();
    rsx! {
        UnitPositionDetail { units, selected_unit, view_navigation }
    }
}

pub fn stories() -> Vec<Story> {
    vec![
        Story::new(
            "Views",
            "CollisionsPage",
            "Positions (empty)",
            collisions_page_positions,
        ),
        Story::new(
            "Views",
            "CollisionsPage",
            "Hotkeys (empty)",
            collisions_page_hotkeys,
        ),
        Story::new(
            "Views",
            "CollisionsPage",
            "Unit positions (empty)",
            collisions_page_unit_positions,
        ),
        Story::single("Views", "ResolvePage", resolve_page_story),
        Story::new(
            "Views",
            "IslandMiniGrid",
            "Top-left",
            island_mini_grid_top_left,
        ),
        Story::new("Views", "IslandMiniGrid", "Center", island_mini_grid_center),
        Story::new(
            "Views",
            "IslandMiniGrid",
            "Bottom-right",
            island_mini_grid_bottom_right,
        ),
        Story::single("Views", "IslandSidebar", island_sidebar_empty),
        Story::single("Views", "IslandDetail", island_detail_empty),
        Story::single("Views", "HotkeyUnitSidebar", hotkey_unit_sidebar_empty),
        Story::single("Views", "HotkeyUnitDetail", hotkey_unit_detail_empty),
        Story::single("Views", "UnitPositionSidebar", unit_position_sidebar_empty),
        Story::single("Views", "UnitPositionDetail", unit_position_detail_empty),
    ]
}
