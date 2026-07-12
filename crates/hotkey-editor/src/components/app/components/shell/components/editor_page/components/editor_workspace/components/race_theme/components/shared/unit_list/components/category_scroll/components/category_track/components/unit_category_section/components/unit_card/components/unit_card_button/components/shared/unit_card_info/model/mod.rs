use super::view::UnitCardInfoView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The card's text column: the display name, the unit id, and whether the card is
/// selected. The id's selected accent is read from `--race-color`, so no race is
/// threaded in.
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
