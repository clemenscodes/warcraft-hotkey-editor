use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct UnitCardIdView {
    pub unit_id: WarcraftObjectId,
    pub is_selected: bool,
}

impl ddd::Layered for UnitCardIdView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UnitCardIdView {}
