use warcraft_api::WarcraftVersion;

#[derive(Clone, PartialEq)]
pub struct SupportedWarcraftVersionView {
    pub label: &'static str,
    pub version: WarcraftVersion,
}

impl ddd::Layered for SupportedWarcraftVersionView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SupportedWarcraftVersionView {}
