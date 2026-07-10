use crate::services::carriers::CarrierUnitView;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`CarriersDialogPanelProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CarriersDialogPanelView {
    pub title: String,
    pub on_close: EventHandler<()>,
    pub carriers: Vec<CarrierUnitView>,
}

impl ddd::View for CarriersDialogPanelView {}
