use super::IslandDetailBody;
use super::model::IslandDetailBodyModel;
use crate::components::app::components::shell::components::collisions_page::presentation::IslandView;
use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`IslandDetailBodyModel`], threaded to this
/// component as data. It is also the detail card's body region: it `impl Render` and renders
/// the presentational `IslandDetailBody` once, so `IslandDetail` places the published `View`
/// directly as `DetailCard`'s body, with no ad-hoc region type.
#[derive(Clone, PartialEq, Default)]
pub struct IslandDetailBodyView {
    pub islands: Vec<IslandView>,
}

impl ddd::View for IslandDetailBodyView {}

impl Render for IslandDetailBodyView {
    type Model = IslandDetailBodyModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        let islands = self.islands.clone();
        rsx! {
            IslandDetailBody {
                islands,
            }
        }
    }
}
