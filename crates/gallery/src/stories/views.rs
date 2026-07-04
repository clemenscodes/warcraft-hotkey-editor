use super::fixtures::{sample_grid_layout, sample_keys};
use dioxus::prelude::*;
use gallery::Story;
use hotkey_editor::components::app::components::shell::components::toasts::ToastMount;

use hotkey_editor::components::app::components::shell::components::collisions_page::CollisionsPage;
use hotkey_editor::components::app::components::shell::components::collisions_page::components::body::components::details::hotkey_unit_detail::HotkeyUnitDetail;
use hotkey_editor::components::app::components::shell::components::collisions_page::components::body::components::sidebars::hotkey_unit_sidebar::HotkeyUnitSidebar;
use hotkey_editor::components::app::components::shell::components::collisions_page::components::body::components::details::island_detail::IslandDetail;
use hotkey_editor::components::app::components::shell::components::collisions_page::components::body::components::sidebars::island_sidebar::IslandSidebar;
use hotkey_editor::components::app::components::shell::components::collisions_page::components::body::components::shared::mini_grid::MiniGrid;
use hotkey_editor::components::app::components::shell::components::collisions_page::components::body::components::details::unit_position_detail::UnitPositionDetail;
use hotkey_editor::components::app::components::shell::components::collisions_page::components::body::components::sidebars::unit_position_sidebar::UnitPositionSidebar;

use hotkey_editor::components::app::components::shell::components::resolve_page::ResolvePage;
use hotkey_editor::services::collision_selection::CollisionSelection;
use hotkey_editor::services::navigation::app_view::AppView;
use hotkey_editor::services::navigation::view_navigation::ViewNavigationContext;
use hotkey_editor::services::resolve_selection::ResolveSelection;
use warcraft_api::Race;
use warcraft_database::UnitMode;
use warcraft_keybinds::{ColumnIndex, GridCoordinate, RowIndex};

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

/// Provide every context the collisions and resolve pages read now that they source
/// their state from context instead of props: the loaded document, the grid layout,
/// the navigation signals, and the per-page selection. The gallery has no router, so
/// this is the app-specific decorator that lets the pages render in isolation — the
/// pages never touch the router themselves.
fn provide_page_contexts() {
    let loaded_keys = use_signal(|| Some(sample_keys()));
    use_context_provider(|| loaded_keys);
    let grid_layout = use_signal(sample_grid_layout);
    use_context_provider(|| grid_layout);
    use_context_provider(make_view_navigation);
    let selected_island = use_signal(|| None::<String>);
    let selected_hotkey_unit = use_signal(|| None::<String>);
    let selected_unit_position = use_signal(|| None::<String>);
    let collision_selection = CollisionSelection {
        selected_island,
        selected_hotkey_unit,
        selected_unit_position,
    };
    use_context_provider(|| collision_selection);
    let selected_move_category = use_signal(|| None::<String>);
    let resolve_selection = ResolveSelection {
        selected_move_category,
    };
    use_context_provider(|| resolve_selection);
}

fn collisions_page_positions() -> Element {
    provide_page_contexts();
    let kind = Some(String::from("positions"));
    rsx! {
        CollisionsPage { kind, entry: None }
    }
}

fn collisions_page_hotkeys() -> Element {
    provide_page_contexts();
    let kind = Some(String::from("hotkeys"));
    rsx! {
        CollisionsPage { kind, entry: None }
    }
}

fn collisions_page_unit_positions() -> Element {
    provide_page_contexts();
    let kind = Some(String::from("unit-positions"));
    rsx! {
        CollisionsPage { kind, entry: None }
    }
}

fn resolve_page_story() -> Element {
    provide_page_contexts();
    rsx! {
        ToastMount {
            ResolvePage { entry: None }
        }
    }
}

fn island_mini_grid_top_left() -> Element {
    let coordinate = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
    rsx! {
        MiniGrid { coordinate }
    }
}

fn island_mini_grid_center() -> Element {
    let coordinate = GridCoordinate::new(ColumnIndex::One, RowIndex::One);
    rsx! {
        MiniGrid { coordinate }
    }
}

fn island_mini_grid_bottom_right() -> Element {
    let coordinate = GridCoordinate::new(ColumnIndex::Three, RowIndex::Two);
    rsx! {
        MiniGrid { coordinate }
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
        Story::new("Views", "MiniGrid", "Top-left", island_mini_grid_top_left),
        Story::new("Views", "MiniGrid", "Center", island_mini_grid_center),
        Story::new(
            "Views",
            "MiniGrid",
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
