use super::view::UnitDetailDialogIntroView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailDialogIntroModel {
    pub portrait_url: Option<String>,
    #[props(into)]
    pub description_text: String,
}

impl From<&UnitDetailDialogIntroView> for UnitDetailDialogIntroModel {
    fn from(view: &UnitDetailDialogIntroView) -> Self {
        let UnitDetailDialogIntroView {
            portrait_url,
            description_text,
        } = view.clone();
        Self {
            portrait_url,
            description_text,
        }
    }
}

impl ddd::Layered for UnitDetailDialogIntroModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for UnitDetailDialogIntroModel {
    type View = UnitDetailDialogIntroView;
}
