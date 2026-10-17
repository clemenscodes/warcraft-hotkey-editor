use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct PagerCardTitleView {
    pub name: String,
    pub unit_id: WarcraftObjectId,
}

impl ddd::Layered for PagerCardTitleView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PagerCardTitleView {}
