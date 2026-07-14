use super::HotkeyUnitDetailBody;
use super::model::HotkeyUnitDetailBodyModel;
use crate::components::app::components::shell::components::collisions_page::presentation::HotkeyUnitView;
use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`HotkeyUnitDetailBodyModel`], threaded to this
/// component as data. It is also the detail card's body region: it `impl Render` and renders
/// the presentational `HotkeyUnitDetailBody` once, so `HotkeyUnitDetail` places the published
/// `View` directly as `DetailCard`'s body, with no ad-hoc region type.
#[derive(Clone, PartialEq, Default)]
pub struct HotkeyUnitDetailBodyView {
    pub units: Vec<HotkeyUnitView>,
}

impl ddd::View for HotkeyUnitDetailBodyView {}

impl Render for HotkeyUnitDetailBodyView {
    type Model = HotkeyUnitDetailBodyModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        let units = self.units.clone();
        rsx! {
            HotkeyUnitDetailBody {
                units,
            }
        }
    }
}
