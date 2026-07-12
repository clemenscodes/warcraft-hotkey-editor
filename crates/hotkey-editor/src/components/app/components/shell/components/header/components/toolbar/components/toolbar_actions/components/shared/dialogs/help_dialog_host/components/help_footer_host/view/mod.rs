use super::HelpFooterHost;
use super::model::HelpFooterHostModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The footer host's published `View`. Fieldless: the component is connected and sources its
/// dismiss wiring from context, so its contract carries no fields. It is also the frame's
/// footer region: it `impl Render` and renders the connected `HelpFooterHost` once, so a
/// dialog places the published `View` directly, with no ad-hoc region type.
#[derive(Clone, PartialEq, Default)]
pub struct HelpFooterHostView;

impl ddd::View for HelpFooterHostView {}

impl Render for HelpFooterHostView {
    type Model = HelpFooterHostModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        rsx! {
            HelpFooterHost {}
        }
    }
}
