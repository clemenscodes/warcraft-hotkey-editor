use super::IslandConflictPanelBody;
use super::model::{IslandConflictCardData, IslandConflictPanelBodyModel};
use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`IslandConflictPanelBodyModel`], threaded to this
/// component as data. It is also the island conflict panel card's body region: it `impl Render`
/// and renders the presentational `IslandConflictPanelBody` once, so `IslandConflictPanel`
/// places the published `View` directly as `PanelCard`'s body, with no ad-hoc region type. The
/// card data is carried as a list so the region is `Default`-able; exactly one is present.
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
            IslandConflictPanelBody { cards }
        }
    }
}
