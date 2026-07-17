use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct NormalUnitCardIdView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for NormalUnitCardIdView {}
