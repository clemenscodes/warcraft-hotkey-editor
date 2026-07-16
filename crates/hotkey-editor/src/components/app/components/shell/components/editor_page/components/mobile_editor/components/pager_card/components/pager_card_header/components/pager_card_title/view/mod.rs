use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct PagerCardTitleView {
    pub name: String,
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for PagerCardTitleView {}
