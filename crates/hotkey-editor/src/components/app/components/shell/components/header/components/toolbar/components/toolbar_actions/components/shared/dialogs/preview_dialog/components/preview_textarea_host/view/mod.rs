use super::PreviewTextareaHost;
use super::model::PreviewTextareaHostModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The preview host's published `View`. Fieldless: the component is connected and sources
/// its data from context, so its contract carries no fields. It is also the frame's body
/// region: it `impl Render` and renders the connected `PreviewTextareaHost` once, so a
/// dialog places the published `View` directly, with no ad-hoc region type.
#[derive(Clone, PartialEq, Default)]
pub struct PreviewTextareaHostView;

impl ddd::View for PreviewTextareaHostView {}

impl Render for PreviewTextareaHostView {
    type Model = PreviewTextareaHostModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        rsx! {
            PreviewTextareaHost {
            


            }
        }
    }
}
