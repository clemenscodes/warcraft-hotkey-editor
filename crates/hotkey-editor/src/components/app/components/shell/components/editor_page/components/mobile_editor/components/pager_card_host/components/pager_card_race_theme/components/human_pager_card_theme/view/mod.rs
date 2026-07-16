use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct HumanPagerCardThemeView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for HumanPagerCardThemeView {}
