use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct PagerCardIdView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::Layered for PagerCardIdView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PagerCardIdView {}
