use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct UndeadPagerCardThemeView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for UndeadPagerCardThemeView {}
