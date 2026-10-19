use super::view::UnitDetailDialogPortraitSlotView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailDialogPortraitSlotModel {
    pub portrait_url: Option<String>,
}

impl From<&UnitDetailDialogPortraitSlotView> for UnitDetailDialogPortraitSlotModel {
    fn from(view: &UnitDetailDialogPortraitSlotView) -> Self {
        let UnitDetailDialogPortraitSlotView { portrait_url } = view.clone();
        Self { portrait_url }
    }
}

impl ddd::Layered for UnitDetailDialogPortraitSlotModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for UnitDetailDialogPortraitSlotModel {
    type View = UnitDetailDialogPortraitSlotView;
}
