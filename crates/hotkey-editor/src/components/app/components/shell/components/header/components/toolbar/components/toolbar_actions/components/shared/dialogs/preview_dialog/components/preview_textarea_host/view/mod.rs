use super::PreviewTextareaHost;
use super::model::PreviewTextareaHostModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub struct PreviewTextareaHostView;

impl ddd::View for PreviewTextareaHostView {}

impl Render for PreviewTextareaHostView {
    type Model = PreviewTextareaHostModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        rsx! {
            PreviewTextareaHost {}
        }
    }
}
