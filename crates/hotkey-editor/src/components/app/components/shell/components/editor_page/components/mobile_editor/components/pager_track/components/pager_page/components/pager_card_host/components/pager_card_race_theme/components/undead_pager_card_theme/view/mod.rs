use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct UndeadPagerCardThemeView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::Layered for UndeadPagerCardThemeView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UndeadPagerCardThemeView {}
