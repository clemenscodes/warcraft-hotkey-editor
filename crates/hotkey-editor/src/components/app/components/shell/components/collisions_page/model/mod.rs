use super::view::CollisionsPageView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CollisionsPageModel {
    pub kind: Option<String>,
    pub entry: Option<String>,
}

impl From<&CollisionsPageView> for CollisionsPageModel {
    fn from(view: &CollisionsPageView) -> Self {
        let CollisionsPageView { kind, entry } = view.clone();
        Self { kind, entry }
    }
}

impl ddd::Model for CollisionsPageModel {
    type View = CollisionsPageView;
}

use super::components::body::{ContentModel, HotkeysPane, PositionsPane, UnitPositionsPane};
use super::presentation::{CollisionUnitView, HotkeyUnitView, IslandView, UnitPositionUnitView};

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

#[derive(Clone, PartialEq)]
pub(super) struct PositionsContent {
    pub(super) has_file: bool,
    pub(super) list: CollisionList<IslandView>,
}

impl From<PositionsContent> for ContentModel {
    fn from(content: PositionsContent) -> Self {
        if !content.has_file {
            let message = super::data::POSITIONS_UPLOAD_PROMPT.to_owned();
            return Self::Empty(message);
        }
        if content.list.unit_count == 0 {
            return Self::Clear;
        }
        let islands = content.list.views;
        let pane = PositionsPane::new(islands);
        let boxed = Box::new(pane);
        Self::Positions(boxed)
    }
}

#[derive(Clone, PartialEq)]
pub(super) struct HotkeysContent {
    pub(super) has_file: bool,
    pub(super) list: CollisionList<HotkeyUnitView>,
}

impl From<HotkeysContent> for ContentModel {
    fn from(content: HotkeysContent) -> Self {
        if !content.has_file {
            let message = super::data::HOTKEYS_UPLOAD_PROMPT.to_owned();
            return Self::Empty(message);
        }
        if content.list.unit_count == 0 {
            return Self::Clear;
        }
        let units = content.list.views;
        let pane = HotkeysPane::new(units);
        let boxed = Box::new(pane);
        Self::Hotkeys(boxed)
    }
}

#[derive(Clone, PartialEq)]
pub(super) struct UnitPositionsContent {
    pub(super) has_file: bool,
    pub(super) list: CollisionList<UnitPositionUnitView>,
}

impl From<UnitPositionsContent> for ContentModel {
    fn from(content: UnitPositionsContent) -> Self {
        if !content.has_file {
            let message = super::data::UNIT_POSITIONS_UPLOAD_PROMPT.to_owned();
            return Self::Empty(message);
        }
        if content.list.unit_count == 0 {
            return Self::Clear;
        }
        let units = content.list.views;
        let pane = UnitPositionsPane::new(units);
        let boxed = Box::new(pane);
        Self::UnitPositions(boxed)
    }
}
