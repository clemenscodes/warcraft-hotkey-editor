use super::view::SupportedWarcraftVersionView;
use dioxus::prelude::*;
use warcraft_api::WarcraftVersion;

#[derive(Props, Clone, PartialEq)]
pub struct SupportedWarcraftVersionModel {
    pub label: &'static str,
    pub version: WarcraftVersion,
}

impl From<&SupportedWarcraftVersionView> for SupportedWarcraftVersionModel {
    fn from(view: &SupportedWarcraftVersionView) -> Self {
        let SupportedWarcraftVersionView { label, version } = view.clone();
        Self { label, version }
    }
}

impl ddd::Layered for SupportedWarcraftVersionModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for SupportedWarcraftVersionModel {
    type View = SupportedWarcraftVersionView;
}
