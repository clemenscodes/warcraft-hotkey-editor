use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct OrcPagerCardThemeView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for OrcPagerCardThemeView {}
