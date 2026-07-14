use super::TemplatesDialogBody;
use super::components::template_gallery::components::template_card::TemplateCardView;
use super::model::TemplatesDialogBodyModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`TemplatesDialogBodyModel`], threaded to this
/// component as data. It is also the templates dialog's body region: it `impl Render` and
/// renders the presentational `TemplatesDialogBody` once, so the host places the published
/// `View` directly as `WarcraftDialog`'s body, with no ad-hoc region type.
#[derive(Clone, PartialEq, Default)]
pub struct TemplatesDialogBodyView {
    pub cards: Vec<TemplateCardView>,
}

impl ddd::View for TemplatesDialogBodyView {}

impl Render for TemplatesDialogBodyView {
    type Model = TemplatesDialogBodyModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        let cards = self.cards.clone();
        rsx! {
            TemplatesDialogBody {
                cards,
            }
        }
    }
}
