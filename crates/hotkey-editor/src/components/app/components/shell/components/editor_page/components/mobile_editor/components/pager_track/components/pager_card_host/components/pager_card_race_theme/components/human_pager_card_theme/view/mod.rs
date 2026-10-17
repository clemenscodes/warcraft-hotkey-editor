use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct HumanPagerCardThemeView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::Layered for HumanPagerCardThemeView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HumanPagerCardThemeView {}
