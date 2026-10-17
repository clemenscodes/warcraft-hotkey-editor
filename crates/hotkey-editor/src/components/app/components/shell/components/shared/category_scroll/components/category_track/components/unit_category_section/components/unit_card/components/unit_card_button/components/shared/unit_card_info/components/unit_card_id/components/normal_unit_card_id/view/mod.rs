use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct NormalUnitCardIdView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::Layered for NormalUnitCardIdView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for NormalUnitCardIdView {}
