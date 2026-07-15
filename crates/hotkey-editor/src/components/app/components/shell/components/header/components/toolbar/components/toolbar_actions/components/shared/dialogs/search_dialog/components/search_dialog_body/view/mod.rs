use super::SearchDialogBody;
use super::model::SearchDialogBodyModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub struct SearchDialogBodyView {}

impl ddd::View for SearchDialogBodyView {}

impl Render for SearchDialogBodyView {
    type Model = SearchDialogBodyModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        rsx! {
            SearchDialogBody {}
        }
    }
}
