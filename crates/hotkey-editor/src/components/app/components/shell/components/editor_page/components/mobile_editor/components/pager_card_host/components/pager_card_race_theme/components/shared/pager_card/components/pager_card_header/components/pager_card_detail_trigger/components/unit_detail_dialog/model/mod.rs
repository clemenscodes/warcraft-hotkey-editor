use super::view::UnitDetailDialogView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailDialogModel {
    pub unit_id: WarcraftObjectId,
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl From<&UnitDetailDialogView> for UnitDetailDialogModel {
    fn from(view: &UnitDetailDialogView) -> Self {
        let UnitDetailDialogView {
            unit_id,
            open,
            on_open_change,
        } = view.clone();
        Self {
            unit_id,
            open,
            on_open_change,
        }
    }
}

impl ddd::Model for UnitDetailDialogModel {
    type View = UnitDetailDialogView;
}
