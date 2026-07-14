use super::EditorTabsBar;
use super::model::EditorTabsBarModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub struct EditorTabsBarView;

impl ddd::View for EditorTabsBarView {}

impl Render for EditorTabsBarView {
    type Model = EditorTabsBarModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        rsx! {
            EditorTabsBar {}
        }
    }
}
