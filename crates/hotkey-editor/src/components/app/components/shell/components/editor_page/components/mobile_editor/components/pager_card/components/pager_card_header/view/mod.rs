use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct PagerCardHeaderView {
    pub icon_url: Option<String>,
    pub name: String,
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for PagerCardHeaderView {}
