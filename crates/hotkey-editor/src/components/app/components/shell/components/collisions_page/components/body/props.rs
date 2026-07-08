use super::components::clear_state::ClearStateProps;
use super::components::details::hotkey_unit_detail::HotkeyUnitDetailProps;
use super::components::details::island_detail::IslandDetailProps;
use super::components::details::unit_position_detail::UnitPositionDetailProps;
use super::components::empty_state::EmptyStateProps;
use super::components::sidebars::island_sidebar::IslandSidebarProps;
use super::components::sidebars::unit_cards_sidebar::UnitCardsSidebarProps;
use crate::components::app::components::shell::components::collisions_page::logic::{
    HotkeyConflictView, UnitPositionConflictView,
};
use dioxus::prelude::*;

/// The position-collision two-pane content: the island sidebar and detail, tagged
/// with the conflict count for the surrounding content wrapper.
#[derive(Clone, PartialEq)]
pub struct PositionsPane {
    count: usize,
    sidebar: IslandSidebarProps,
    detail: IslandDetailProps,
}

impl PositionsPane {
    pub fn new(count: usize, sidebar: IslandSidebarProps, detail: IslandDetailProps) -> Self {
        Self {
            count,
            sidebar,
            detail,
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn sidebar(&self) -> &IslandSidebarProps {
        &self.sidebar
    }

    pub fn detail(&self) -> &IslandDetailProps {
        &self.detail
    }
}

/// The hotkey-collision two-pane content: the clashing-units sidebar and detail.
#[derive(Clone, PartialEq)]
pub struct HotkeysPane {
    count: usize,
    sidebar: UnitCardsSidebarProps<HotkeyConflictView>,
    detail: HotkeyUnitDetailProps,
}

impl HotkeysPane {
    pub fn new(
        count: usize,
        sidebar: UnitCardsSidebarProps<HotkeyConflictView>,
        detail: HotkeyUnitDetailProps,
    ) -> Self {
        Self {
            count,
            sidebar,
            detail,
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn sidebar(&self) -> &UnitCardsSidebarProps<HotkeyConflictView> {
        &self.sidebar
    }

    pub fn detail(&self) -> &HotkeyUnitDetailProps {
        &self.detail
    }
}

/// The per-unit position-collision two-pane content.
#[derive(Clone, PartialEq)]
pub struct UnitPositionsPane {
    count: usize,
    sidebar: UnitCardsSidebarProps<UnitPositionConflictView>,
    detail: UnitPositionDetailProps,
}

impl UnitPositionsPane {
    pub fn new(
        count: usize,
        sidebar: UnitCardsSidebarProps<UnitPositionConflictView>,
        detail: UnitPositionDetailProps,
    ) -> Self {
        Self {
            count,
            sidebar,
            detail,
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn sidebar(&self) -> &UnitCardsSidebarProps<UnitPositionConflictView> {
        &self.sidebar
    }

    pub fn detail(&self) -> &UnitPositionDetailProps {
        &self.detail
    }
}

/// The active collision content as data: an upload prompt, an all-clear state, or
/// one of the three kinds' two-pane views. `Body` renders each variant;
/// the hook only shapes it.
#[derive(Clone, PartialEq)]
pub enum ContentModel {
    Empty(EmptyStateProps),
    Clear(ClearStateProps),
    Positions(Box<PositionsPane>),
    Hotkeys(Box<HotkeysPane>),
    UnitPositions(Box<UnitPositionsPane>),
}

/// The dispatcher's input: the shaped active content for the current kind and state.
#[derive(Props, Clone, PartialEq)]
pub struct BodyProps {
    pub content: ContentModel,
}
