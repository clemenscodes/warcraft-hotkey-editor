use super::components::body::components::clear_state::ClearStateProps;
use super::components::body::components::details::hotkey_unit_detail::HotkeyUnitDetailProps;
use super::components::body::components::details::island_detail::IslandDetailProps;
use super::components::body::components::details::unit_position_detail::UnitPositionDetailProps;
use super::components::body::components::empty_state::EmptyStateProps;
use super::components::body::components::sidebars::island_sidebar::IslandSidebarProps;
use super::components::body::components::sidebars::unit_cards_sidebar::UnitCardsSidebarProps;
use super::components::body::{ContentModel, HotkeysPane, PositionsPane, UnitPositionsPane};
use super::logic::{CollisionUnitView, HotkeyUnitView, IslandView, UnitPositionUnitView};
use crate::services::navigation::app_view::CollisionKind;

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

impl<Conflict> CollisionEntry for CollisionUnitView<Conflict> {
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
/// file is loaded and the island list. The sidebar and detail read the selection and
/// the navigation from context, so neither is carried here.
#[derive(Clone, PartialEq)]
pub(super) struct PositionsContent {
    pub(super) has_file: bool,
    pub(super) list: CollisionList<IslandView>,
}

impl From<PositionsContent> for ContentModel {
    fn from(content: PositionsContent) -> Self {
        if !content.has_file {
            let state = EmptyStateProps {
                collision_kind: CollisionKind::Positions,
                message: super::data::POSITIONS_UPLOAD_PROMPT.to_owned(),
            };
            return Self::Empty(state);
        }
        if content.list.unit_count == 0 {
            let state = ClearStateProps {
                collision_kind: CollisionKind::Positions,
            };
            return Self::Clear(state);
        }
        let sidebar_islands = content.list.views.clone();
        let sidebar = IslandSidebarProps {
            islands: sidebar_islands,
        };
        let detail = IslandDetailProps {
            islands: content.list.views,
        };
        let count = content.list.unit_count;
        let pane = PositionsPane::new(count, sidebar, detail);
        let boxed = Box::new(pane);
        Self::Positions(boxed)
    }
}

/// The inputs that resolve the hotkey-collision kind's active content.
#[derive(Clone, PartialEq)]
pub(super) struct HotkeysContent {
    pub(super) has_file: bool,
    pub(super) list: CollisionList<HotkeyUnitView>,
}

impl From<HotkeysContent> for ContentModel {
    fn from(content: HotkeysContent) -> Self {
        if !content.has_file {
            let state = EmptyStateProps {
                collision_kind: CollisionKind::Hotkeys,
                message: super::data::HOTKEYS_UPLOAD_PROMPT.to_owned(),
            };
            return Self::Empty(state);
        }
        if content.list.unit_count == 0 {
            let state = ClearStateProps {
                collision_kind: CollisionKind::Hotkeys,
            };
            return Self::Clear(state);
        }
        let sidebar_units = content.list.views.clone();
        let sidebar = UnitCardsSidebarProps {
            units: sidebar_units,
        };
        let detail = HotkeyUnitDetailProps {
            units: content.list.views,
        };
        let count = content.list.unit_count;
        let pane = HotkeysPane::new(count, sidebar, detail);
        let boxed = Box::new(pane);
        Self::Hotkeys(boxed)
    }
}

/// The inputs that resolve the per-unit position-collision kind's active content.
#[derive(Clone, PartialEq)]
pub(super) struct UnitPositionsContent {
    pub(super) has_file: bool,
    pub(super) list: CollisionList<UnitPositionUnitView>,
}

impl From<UnitPositionsContent> for ContentModel {
    fn from(content: UnitPositionsContent) -> Self {
        if !content.has_file {
            let state = EmptyStateProps {
                collision_kind: CollisionKind::UnitPositions,
                message: super::data::UNIT_POSITIONS_UPLOAD_PROMPT.to_owned(),
            };
            return Self::Empty(state);
        }
        if content.list.unit_count == 0 {
            let state = ClearStateProps {
                collision_kind: CollisionKind::UnitPositions,
            };
            return Self::Clear(state);
        }
        let sidebar_units = content.list.views.clone();
        let sidebar = UnitCardsSidebarProps {
            units: sidebar_units,
        };
        let detail = UnitPositionDetailProps {
            units: content.list.views,
        };
        let count = content.list.unit_count;
        let pane = UnitPositionsPane::new(count, sidebar, detail);
        let boxed = Box::new(pane);
        Self::UnitPositions(boxed)
    }
}
