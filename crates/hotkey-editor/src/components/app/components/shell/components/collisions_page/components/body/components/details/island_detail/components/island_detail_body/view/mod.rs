use super::IslandDetailBody;
use super::model::IslandDetailBodyModel;
use crate::components::app::components::shell::components::collisions_page::presentation::IslandView;
use browser_kit::frame::Render;
use dioxus::prelude::*;

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
