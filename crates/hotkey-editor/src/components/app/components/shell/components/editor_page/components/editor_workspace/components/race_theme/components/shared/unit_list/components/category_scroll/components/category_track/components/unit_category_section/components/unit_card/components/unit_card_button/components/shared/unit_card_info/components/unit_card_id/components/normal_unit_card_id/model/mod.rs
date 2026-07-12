use super::view::NormalUnitCardIdView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The unit id the muted code element renders.
#[derive(Props, Clone, PartialEq)]
pub struct NormalUnitCardIdModel {
    pub unit_id: WarcraftObjectId,
}

impl From<&NormalUnitCardIdView> for NormalUnitCardIdModel {
    fn from(view: &NormalUnitCardIdView) -> Self {
        let NormalUnitCardIdView { unit_id } = view.clone();
        Self { unit_id }
    }
}

impl ddd::Model for NormalUnitCardIdModel {
    type View = NormalUnitCardIdView;
}
