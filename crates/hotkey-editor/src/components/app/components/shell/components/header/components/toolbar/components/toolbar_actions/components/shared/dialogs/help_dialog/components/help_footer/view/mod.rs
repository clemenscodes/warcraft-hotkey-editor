use super::HelpFooter;
use super::model::HelpFooterModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The footer host's published `View`: the dismiss handler fired by the button, threaded in
/// as data by the dialog that owns the open signal. `Callback<MouseEvent>` so the footer
/// region can carry it as `Default` plain data. It is also the frame's footer region: it
/// `impl Render` and renders the `HelpFooter` once, so a dialog places the published `View`
/// directly, with no ad-hoc region type.
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
            HelpFooter { on_dismiss }
        }
    }
}
