use dioxus::prelude::*;

use super::components::body::components::clear_state::ClearStateProps;
use super::components::body::components::details::hotkey_unit_detail::HotkeyUnitDetailProps;
use super::components::body::components::details::island_detail::IslandDetailProps;
use super::components::body::components::details::unit_position_detail::UnitPositionDetailProps;
use super::components::body::components::empty_state::EmptyStateProps;
use super::components::body::components::sidebars::hotkey_unit_sidebar::HotkeyUnitSidebarProps;
use super::components::body::components::sidebars::island_sidebar::IslandSidebarProps;
use super::components::body::components::sidebars::unit_position_sidebar::UnitPositionSidebarProps;
use super::components::body::{ContentModel, HotkeysPane, PositionsPane, UnitPositionsPane};
use super::logic::{HotkeyUnitView, IslandView, UnitPositionUnitView};
use crate::services::navigation::view_navigation::ViewNavigationContext;

/// A collision view that carries a stable selection key and a conflict count.
/// Formalises the two facts every collision list needs — its selection identity
/// and how many conflicts it holds — so the list derivation and the selection
/// validity hook can be written once over any kind.
pub(super) trait CollisionEntry {
    fn key(&self) -> &str;
    fn collision_count(&self) -> usize;
}

impl CollisionEntry for IslandView {
    fn key(&self) -> &str {
        self.key()
    }

    fn collision_count(&self) -> usize {
        self.collision_count()
    }
}

impl CollisionEntry for HotkeyUnitView {
    fn key(&self) -> &str {
        self.key()
    }

    fn collision_count(&self) -> usize {
        self.collision_count()
    }
}

impl CollisionEntry for UnitPositionUnitView {
    fn key(&self) -> &str {
        self.key()
    }

    fn collision_count(&self) -> usize {
        self.collision_count()
    }
}

/// One kind's collision list with its two summary counts already derived: the
/// number of affected units (`unit_count`) and the total number of conflicts
/// across them (`collision_count`).
#[derive(Clone, PartialEq)]
pub(super) struct CollisionList<View> {
    pub(super) views: Vec<View>,
    pub(super) unit_count: usize,
    pub(super) collision_count: usize,
}

impl<View: CollisionEntry> From<Vec<View>> for CollisionList<View> {
    fn from(views: Vec<View>) -> Self {
        let unit_count = views.len();
        let collision_count: usize = views.iter().map(CollisionEntry::collision_count).sum();
        Self {
            views,
            unit_count,
            collision_count,
        }
    }
}

/// The inputs that resolve the position-collision kind's active content: whether a
/// file is loaded, the island list, the selection signal, and the navigation
/// context the detail links use.
#[derive(Clone, PartialEq)]
pub(super) struct PositionsContent {
    pub(super) has_file: bool,
    pub(super) list: CollisionList<IslandView>,
    pub(super) selected_island: Signal<Option<String>>,
    pub(super) view_navigation: ViewNavigationContext,
}

impl From<PositionsContent> for ContentModel {
    fn from(content: PositionsContent) -> Self {
        if !content.has_file {
            let state = EmptyStateProps {
                collision_kind: "positions",
                message: super::data::POSITIONS_UPLOAD_PROMPT.to_owned(),
            };
            return Self::Empty(state);
        }
        if content.list.unit_count == 0 {
            let state = ClearStateProps {
                collision_kind: "positions",
            };
            return Self::Clear(state);
        }
        let sidebar_islands = content.list.views.clone();
        let sidebar = IslandSidebarProps {
            islands: sidebar_islands,
            selected_island: content.selected_island,
        };
        let detail = IslandDetailProps {
            islands: content.list.views,
            selected_island: content.selected_island,
            view_navigation: content.view_navigation,
        };
        let pane = PositionsPane {
            collision_kind: "positions",
            count: content.list.unit_count,
            sidebar,
            detail,
        };
        let boxed = Box::new(pane);
        Self::Positions(boxed)
    }
}

/// The inputs that resolve the hotkey-collision kind's active content.
#[derive(Clone, PartialEq)]
pub(super) struct HotkeysContent {
    pub(super) has_file: bool,
    pub(super) list: CollisionList<HotkeyUnitView>,
    pub(super) selected_unit: Signal<Option<String>>,
    pub(super) view_navigation: ViewNavigationContext,
}

impl From<HotkeysContent> for ContentModel {
    fn from(content: HotkeysContent) -> Self {
        if !content.has_file {
            let state = EmptyStateProps {
                collision_kind: "hotkeys",
                message: super::data::HOTKEYS_UPLOAD_PROMPT.to_owned(),
            };
            return Self::Empty(state);
        }
        if content.list.unit_count == 0 {
            let state = ClearStateProps {
                collision_kind: "hotkeys",
            };
            return Self::Clear(state);
        }
        let sidebar_units = content.list.views.clone();
        let sidebar = HotkeyUnitSidebarProps {
            units: sidebar_units,
            selected_unit: content.selected_unit,
        };
        let detail = HotkeyUnitDetailProps {
            units: content.list.views,
            selected_unit: content.selected_unit,
            view_navigation: content.view_navigation,
        };
        let pane = HotkeysPane {
            collision_kind: "hotkeys",
            count: content.list.unit_count,
            sidebar,
            detail,
        };
        let boxed = Box::new(pane);
        Self::Hotkeys(boxed)
    }
}

/// The inputs that resolve the per-unit position-collision kind's active content.
#[derive(Clone, PartialEq)]
pub(super) struct UnitPositionsContent {
    pub(super) has_file: bool,
    pub(super) list: CollisionList<UnitPositionUnitView>,
    pub(super) selected_unit: Signal<Option<String>>,
    pub(super) view_navigation: ViewNavigationContext,
}

impl From<UnitPositionsContent> for ContentModel {
    fn from(content: UnitPositionsContent) -> Self {
        if !content.has_file {
            let state = EmptyStateProps {
                collision_kind: "unit-positions",
                message: super::data::UNIT_POSITIONS_UPLOAD_PROMPT.to_owned(),
            };
            return Self::Empty(state);
        }
        if content.list.unit_count == 0 {
            let state = ClearStateProps {
                collision_kind: "unit-positions",
            };
            return Self::Clear(state);
        }
        let sidebar_units = content.list.views.clone();
        let sidebar = UnitPositionSidebarProps {
            units: sidebar_units,
            selected_unit: content.selected_unit,
        };
        let detail = UnitPositionDetailProps {
            units: content.list.views,
            selected_unit: content.selected_unit,
            view_navigation: content.view_navigation,
        };
        let pane = UnitPositionsPane {
            collision_kind: "unit-positions",
            count: content.list.unit_count,
            sidebar,
            detail,
        };
        let boxed = Box::new(pane);
        Self::UnitPositions(boxed)
    }
}
