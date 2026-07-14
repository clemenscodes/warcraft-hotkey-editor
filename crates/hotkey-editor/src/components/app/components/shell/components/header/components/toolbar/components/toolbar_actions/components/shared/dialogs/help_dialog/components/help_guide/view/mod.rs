use super::HelpGuide;
use super::model::HelpGuideModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub struct HelpGuideView;

impl ddd::View for HelpGuideView {}

impl Render for HelpGuideView {
    type Model = HelpGuideModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        rsx! {
            HelpGuide {}
        }
    }
}
