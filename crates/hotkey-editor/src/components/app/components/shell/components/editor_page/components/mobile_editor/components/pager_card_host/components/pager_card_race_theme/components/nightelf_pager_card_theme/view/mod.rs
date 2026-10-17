use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct NightelfPagerCardThemeView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::Layered for NightelfPagerCardThemeView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for NightelfPagerCardThemeView {}
