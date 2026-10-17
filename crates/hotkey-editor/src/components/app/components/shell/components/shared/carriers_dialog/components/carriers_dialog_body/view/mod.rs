use super::CarriersDialogBody;
use super::model::CarriersDialogBodyModel;
use crate::services::carriers::CarrierUnitView;
use browser_kit::frame::Render;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub struct CarriersDialogBodyView {
    pub carriers: Vec<CarrierUnitView>,
}

impl ddd::Layered for CarriersDialogBodyView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for CarriersDialogBodyView {}

impl Render for CarriersDialogBodyView {
    type Model = CarriersDialogBodyModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        let carriers = self.carriers.clone();
        rsx! {
            CarriersDialogBody {
                carriers,
            }
        }
    }
}
