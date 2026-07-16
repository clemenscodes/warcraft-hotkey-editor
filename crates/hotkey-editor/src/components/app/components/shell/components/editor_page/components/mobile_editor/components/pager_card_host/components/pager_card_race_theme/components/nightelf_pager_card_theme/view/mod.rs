use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct NightelfPagerCardThemeView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for NightelfPagerCardThemeView {}
