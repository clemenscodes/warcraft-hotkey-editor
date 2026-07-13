use super::HelpFooter;
use super::model::HelpFooterModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The footer host's published `View`. Fieldless: the component is connected and sources its
/// dismiss wiring from context, so its contract carries no fields. It is also the frame's
/// footer region: it `impl Render` and renders the connected `HelpFooter` once, so a
/// dialog places the published `View` directly, with no ad-hoc region type.
#[derive(Clone, PartialEq, Default)]
pub struct HelpFooterView;

impl ddd::View for HelpFooterView {}

impl Render for HelpFooterView {
    type Model = HelpFooterModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        rsx! {
            HelpFooter {}
        }
    }
}
