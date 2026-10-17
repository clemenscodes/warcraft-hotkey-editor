use super::SystemHotkeysDialogBody;
use super::model::SystemHotkeysDialogBodyModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub struct SystemHotkeysDialogBodyView;

impl ddd::Layered for SystemHotkeysDialogBodyView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SystemHotkeysDialogBodyView {}

impl Render for SystemHotkeysDialogBodyView {
    type Model = SystemHotkeysDialogBodyModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        rsx! {
            SystemHotkeysDialogBody {}
        }
    }
}
