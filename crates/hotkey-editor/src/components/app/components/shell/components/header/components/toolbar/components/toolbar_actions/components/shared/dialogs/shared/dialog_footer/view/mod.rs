use super::DialogFooter;
use super::model::DialogFooterModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub struct DialogFooterView {
    pub label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for DialogFooterView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for DialogFooterView {}

impl Render for DialogFooterView {
    type Model = DialogFooterModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        let label = self.label;
        let onclick = self.onclick;
        rsx! {
            DialogFooter {
                label,
                onclick,
            }
        }
    }
}
