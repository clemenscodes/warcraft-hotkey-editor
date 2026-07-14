use super::HelpFooter;
use super::model::HelpFooterModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub struct HelpFooterView {
    pub on_dismiss: Callback<MouseEvent>,
}

impl ddd::View for HelpFooterView {}

impl Render for HelpFooterView {
    type Model = HelpFooterModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        let on_dismiss = self.on_dismiss;
        rsx! {
            HelpFooter {
                on_dismiss,
            }
        }
    }
}
