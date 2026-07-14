use super::CarriersDialogBody;
use super::model::CarriersDialogBodyModel;
use crate::services::carriers::CarrierUnitView;
use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`CarriersDialogBodyModel`], threaded to this
/// component as data. It is also the carriers dialog's body region: it `impl Render` and
/// renders the presentational `CarriersDialogBody` once, so the host places the published
/// `View` directly as `WarcraftDialog`'s body, with no ad-hoc region type.
#[derive(Clone, PartialEq, Default)]
pub struct CarriersDialogBodyView {
    pub carriers: Vec<CarrierUnitView>,
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
