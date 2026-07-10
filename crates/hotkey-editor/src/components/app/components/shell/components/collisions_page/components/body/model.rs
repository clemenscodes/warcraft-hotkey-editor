use crate::components::app::components::shell::components::collisions_page::logic::{
    HotkeyUnitView, IslandView, UnitPositionUnitView,
};

/// The position-collision two-pane content: the island list shared by the sidebar and
/// the detail pane.
#[derive(Clone, PartialEq)]
pub struct PositionsPane {
    islands: Vec<IslandView>,
}

impl PositionsPane {
    pub fn new(islands: Vec<IslandView>) -> Self {
        Self { islands }
    }

    pub fn islands(&self) -> &[IslandView] {
        &self.islands
    }
}

/// The hotkey-collision two-pane content: the clashing-unit list shared by the sidebar
/// and the detail pane.
#[derive(Clone, PartialEq)]
pub struct HotkeysPane {
    units: Vec<HotkeyUnitView>,
}

impl HotkeysPane {
    pub fn new(units: Vec<HotkeyUnitView>) -> Self {
        Self { units }
    }

    pub fn units(&self) -> &[HotkeyUnitView] {
        &self.units
    }
}

/// The per-unit position-collision two-pane content: the clashing-unit list shared by
/// the sidebar and the detail pane.
#[derive(Clone, PartialEq)]
pub struct UnitPositionsPane {
    units: Vec<UnitPositionUnitView>,
}

impl UnitPositionsPane {
    pub fn new(units: Vec<UnitPositionUnitView>) -> Self {
        Self { units }
    }

    pub fn units(&self) -> &[UnitPositionUnitView] {
        &self.units
    }
}

/// The active collision content as data: an upload prompt (its message), an all-clear
/// state, or one of the three kinds' two-pane views. `Body` renders each variant; the
/// hook only shapes it.
#[derive(Clone, PartialEq)]
pub enum ContentModel {
    Empty(String),
    Clear,
    Positions(Box<PositionsPane>),
    Hotkeys(Box<HotkeysPane>),
    UnitPositions(Box<UnitPositionsPane>),
}
