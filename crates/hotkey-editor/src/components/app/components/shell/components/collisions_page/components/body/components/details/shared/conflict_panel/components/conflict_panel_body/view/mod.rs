use super::ConflictPanelBody;
use super::model::ConflictPanelBodyModel;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_model::ConflictCardModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub struct ConflictPanelBodyView {
    pub(crate) models: Vec<ConflictCardModel>,
}

impl ddd::View for ConflictPanelBodyView {}

impl Render for ConflictPanelBodyView {
    type Model = ConflictPanelBodyModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        let models = self.models.clone();
        rsx! {
            ConflictPanelBody {
                models,
            }
        }
    }
}
