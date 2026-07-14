use super::SystemHotkeysDialogBody;
use super::model::SystemHotkeysDialogBodyModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The system-hotkeys body's published `View`. Fieldless: the component is connected and
/// sources its state from context, so its contract carries no fields. It is also the frame's
/// body region: it `impl Render` and renders the connected `SystemHotkeysDialogBody` once, so
/// the dialog places the published `View` directly, with no ad-hoc region type.
#[derive(Clone, PartialEq, Default)]
pub struct SystemHotkeysDialogBodyView;

impl ddd::View for SystemHotkeysDialogBodyView {}

impl Render for SystemHotkeysDialogBodyView {
    type Model = SystemHotkeysDialogBodyModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        rsx! {
            SystemHotkeysDialogBody {
            


            }
        }
    }
}
