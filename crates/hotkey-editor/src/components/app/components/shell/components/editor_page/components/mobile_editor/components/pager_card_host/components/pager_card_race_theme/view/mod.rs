use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct PagerCardRaceThemeView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for PagerCardRaceThemeView {}
