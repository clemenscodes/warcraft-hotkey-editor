use super::view::NormalUnitCardIdView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The unit id the muted code element renders.
#[derive(Props, Clone, PartialEq)]
pub struct NormalUnitCardIdProps {
    pub unit_id: WarcraftObjectId,
}

impl From<&NormalUnitCardIdView> for NormalUnitCardIdProps {
    fn from(view: &NormalUnitCardIdView) -> Self {
        let NormalUnitCardIdView { unit_id } = view.clone();
        Self { unit_id }
    }
}

impl ddd::Props for NormalUnitCardIdProps {
    type View = NormalUnitCardIdView;
}
