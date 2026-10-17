use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct UnitIdView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::Layered for UnitIdView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UnitIdView {}
