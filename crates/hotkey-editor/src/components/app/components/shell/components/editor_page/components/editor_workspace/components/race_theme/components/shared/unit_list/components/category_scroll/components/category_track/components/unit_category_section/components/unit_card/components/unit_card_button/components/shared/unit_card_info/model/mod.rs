use super::view::UnitCardInfoView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct UnitCardInfoModel {
    #[props(into)]
    pub display_name: String,
    pub unit_id: WarcraftObjectId,
    pub is_selected: bool,
}

impl From<&UnitCardInfoView> for UnitCardInfoModel {
    fn from(view: &UnitCardInfoView) -> Self {
        let UnitCardInfoView {
            display_name,
            unit_id,
            is_selected,
        } = view.clone();
        Self {
            display_name,
            unit_id,
            is_selected,
        }
    }
}

impl ddd::Model for UnitCardInfoModel {
    type View = UnitCardInfoView;
}
