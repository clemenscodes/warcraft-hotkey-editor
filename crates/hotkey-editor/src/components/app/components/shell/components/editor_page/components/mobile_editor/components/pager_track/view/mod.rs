use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct PagerTrackView {
    pub unit_ids: Vec<WarcraftObjectId>,
}

impl ddd::Layered for PagerTrackView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PagerTrackView {}
