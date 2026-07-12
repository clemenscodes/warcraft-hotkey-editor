use super::ConflictPanelBody;
use super::model::ConflictPanelBodyModel;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_model::ConflictCardModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`ConflictPanelBodyModel`], threaded to this
/// component as data. It is also the conflict panel card's body region: it `impl Render` and
/// renders the presentational `ConflictPanelBody` once, so `ConflictPanel` places the
/// published `View` directly as `PanelCard`'s body, with no ad-hoc region type. The shaped
/// card model is carried as a list so the region is `Default`-able; exactly one is present.
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
            ConflictPanelBody { models }
        }
    }
}
