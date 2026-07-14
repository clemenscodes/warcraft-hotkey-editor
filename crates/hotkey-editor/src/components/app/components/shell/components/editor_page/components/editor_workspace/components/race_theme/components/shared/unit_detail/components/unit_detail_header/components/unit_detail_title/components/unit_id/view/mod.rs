use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct UnitIdView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for UnitIdView {}
