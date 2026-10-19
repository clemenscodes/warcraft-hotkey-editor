use warcraft_api::WarcraftObjectId;

/// The units one page of the pager shows side by side: one on a phone or a
/// tablet, two on a laptop.
#[derive(Clone, PartialEq)]
pub struct PagerPageView {
    pub unit_ids: Vec<WarcraftObjectId>,
}

impl ddd::Layered for PagerPageView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PagerPageView {}
