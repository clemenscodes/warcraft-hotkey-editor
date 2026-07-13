use super::HelpGuide;
use super::model::HelpGuideModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The guide host's published `View`. Fieldless: the component is connected and sources its
/// static content, so its contract carries no fields. It is also the frame's body region: it
/// `impl Render` and renders the connected `HelpGuide` once, so a dialog places the
/// published `View` directly, with no ad-hoc region type.
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
