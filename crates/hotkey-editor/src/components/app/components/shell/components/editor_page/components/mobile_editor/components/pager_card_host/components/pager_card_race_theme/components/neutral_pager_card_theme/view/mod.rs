use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct NeutralPagerCardThemeView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::Layered for NeutralPagerCardThemeView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for NeutralPagerCardThemeView {}
