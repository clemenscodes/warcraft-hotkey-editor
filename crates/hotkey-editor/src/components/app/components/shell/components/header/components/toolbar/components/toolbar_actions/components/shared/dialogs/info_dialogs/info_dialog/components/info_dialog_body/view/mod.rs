use super::InfoDialogBody;
use super::model::InfoDialogBodyModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub struct InfoDialogBodyView {
    pub intro: &'static str,
    pub warning: Option<&'static str>,
}

impl ddd::Layered for InfoDialogBodyView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for InfoDialogBodyView {}

impl Render for InfoDialogBodyView {
    type Model = InfoDialogBodyModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        let intro = self.intro;
        let warning = self.warning;
        rsx! {
            InfoDialogBody {
                intro,
                warning,
            }
        }
    }
}
