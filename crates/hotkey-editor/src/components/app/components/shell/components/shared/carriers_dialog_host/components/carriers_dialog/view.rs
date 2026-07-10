use crate::services::carriers::{CarrierUnitView, InspectedAbility};
use dioxus::prelude::*;

/// The published `View` contract mirroring [`CarriersDialogProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CarriersDialogView {
    pub title: String,
    pub carriers: Vec<CarrierUnitView>,
    pub open_state: Signal<Option<InspectedAbility>>,
}

impl ddd::View for CarriersDialogView {}
