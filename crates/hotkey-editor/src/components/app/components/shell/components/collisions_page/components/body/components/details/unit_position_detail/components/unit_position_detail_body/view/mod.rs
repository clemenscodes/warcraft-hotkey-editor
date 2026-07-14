use super::UnitPositionDetailBody;
use super::model::UnitPositionDetailBodyModel;
use crate::components::app::components::shell::components::collisions_page::presentation::UnitPositionUnitView;
use browser_kit::frame::Render;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub struct UnitPositionDetailBodyView {
    pub units: Vec<UnitPositionUnitView>,
}

impl ddd::View for UnitPositionDetailBodyView {}

impl Render for UnitPositionDetailBodyView {
    type Model = UnitPositionDetailBodyModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        let units = self.units.clone();
        rsx! {
            UnitPositionDetailBody {
                units,
            }
        }
    }
}
