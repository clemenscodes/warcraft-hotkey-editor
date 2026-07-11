use super::view::CollisionsPageView;
use dioxus::prelude::*;

/// The collisions page's route parameters: the active collision `?kind=` and the
/// selected list `?entry=`. That is the page's entire URL state — the editor
/// selection is the editor's, not the collisions page's, so it is not carried here;
/// it persists in the shell's signals while this page is shown and reappears in the
/// URL when the editor is next active.
///
/// The per-kind selection signals live in the shell (backing the `?entry=` param, one
/// per kind for per-tab memory) so they outlive leaving the page and feed the URL
/// sync; the page reaches them through context, never the router.
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

/// The inputs that resolve the hotkey-collision kind's active content.
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

/// The inputs that resolve the per-unit position-collision kind's active content.
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
