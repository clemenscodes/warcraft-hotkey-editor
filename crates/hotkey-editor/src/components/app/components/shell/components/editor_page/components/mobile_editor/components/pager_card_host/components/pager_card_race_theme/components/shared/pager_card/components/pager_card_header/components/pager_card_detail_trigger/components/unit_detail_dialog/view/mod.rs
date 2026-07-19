use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct UnitDetailDialogView {
    pub unit_id: WarcraftObjectId,
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl ddd::View for UnitDetailDialogView {}
