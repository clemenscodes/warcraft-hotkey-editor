use super::IslandConflictPanelBody;
use super::model::{IslandConflictCardData, IslandConflictPanelBodyModel};
use browser_kit::frame::Render;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub struct IslandConflictPanelBodyView {
    pub(crate) cards: Vec<IslandConflictCardData>,
}

impl ddd::View for IslandConflictPanelBodyView {}

impl Render for IslandConflictPanelBodyView {
    type Model = IslandConflictPanelBodyModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        let cards = self.cards.clone();
        rsx! {
            IslandConflictPanelBody {
                cards,
            }
        }
    }
}
