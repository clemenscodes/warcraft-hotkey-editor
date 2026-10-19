use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct SelectedUnitCardIdView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::Layered for SelectedUnitCardIdView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SelectedUnitCardIdView {}
