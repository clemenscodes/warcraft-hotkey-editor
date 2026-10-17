use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct PagerCardRaceThemeView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::Layered for PagerCardRaceThemeView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PagerCardRaceThemeView {}
