use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct OrcPagerCardThemeView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::Layered for OrcPagerCardThemeView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for OrcPagerCardThemeView {}
