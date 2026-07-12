use super::InfoDialogBody;
use super::model::InfoDialogBodyModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`InfoDialogBodyModel`], threaded to this
/// component as data. It is also the info dialog's body region: it `impl Render` and
/// renders the presentational `InfoDialogBody` once, so the base places the published
/// `View` directly as `WarcraftDialog`'s body, with no ad-hoc region type.
#[derive(Clone, PartialEq, Default)]
pub struct InfoDialogBodyView {
    pub intro: &'static str,
    pub warning: Option<&'static str>,
    pub primary_label: &'static str,
    pub on_primary: EventHandler<MouseEvent>,
    pub on_cancel: EventHandler<MouseEvent>,
}

impl ddd::View for InfoDialogBodyView {}

impl Render for InfoDialogBodyView {
    type Model = InfoDialogBodyModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        let intro = self.intro;
        let warning = self.warning;
        let primary_label = self.primary_label;
        let on_primary = self.on_primary;
        let on_cancel = self.on_cancel;
        rsx! {
            InfoDialogBody {
                intro,
                warning,
                primary_label,
                on_primary,
                on_cancel,
            }
        }
    }
}
