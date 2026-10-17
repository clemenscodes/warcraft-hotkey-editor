use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct PagerCardHostView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::Layered for PagerCardHostView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PagerCardHostView {}
