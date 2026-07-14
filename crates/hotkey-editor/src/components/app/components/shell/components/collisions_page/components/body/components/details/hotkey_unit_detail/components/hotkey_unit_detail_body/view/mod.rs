use super::HotkeyUnitDetailBody;
use super::model::HotkeyUnitDetailBodyModel;
use crate::components::app::components::shell::components::collisions_page::presentation::HotkeyUnitView;
use browser_kit::frame::Render;
use dioxus::prelude::*;

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
