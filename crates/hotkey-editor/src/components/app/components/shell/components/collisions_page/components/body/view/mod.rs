use super::Body;
use super::model::BodyModel;
use crate::components::app::components::shell::components::collisions_page::presentation::{
    HotkeyUnitView, IslandView, UnitPositionUnitView,
};
use browser_kit::frame::Render;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub struct BodyView {
    pub content: ContentModel,
}

impl ddd::View for BodyView {}

impl Render for BodyView {
    type Model = BodyModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        let content = self.content.clone();
        rsx! {
            Body {
                content,
            }
        }
    }
}

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

#[derive(Clone, PartialEq, Default)]
pub enum ContentModel {
    Empty(String),
    #[default]
    Clear,
    Positions(Box<PositionsPane>),
    Hotkeys(Box<HotkeysPane>),
    UnitPositions(Box<UnitPositionsPane>),
}
