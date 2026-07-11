use super::view::UnitIdView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The unit's database id, shown as a monospace caption.
#[derive(Props, Clone, PartialEq)]
pub struct UnitIdModel {
    pub unit_id: WarcraftObjectId,
}

impl From<&UnitIdView> for UnitIdModel {
    fn from(view: &UnitIdView) -> Self {
        let UnitIdView { unit_id } = view.clone();
        Self { unit_id }
    }
}

impl ddd::Model for UnitIdModel {
    type View = UnitIdView;
}
