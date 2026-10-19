use super::view::SystemHotkeysDialogBodyView;

#[derive(Clone, PartialEq, Default)]
pub struct SystemHotkeysDialogBodyModel;

impl From<&SystemHotkeysDialogBodyView> for SystemHotkeysDialogBodyModel {
    fn from(_view: &SystemHotkeysDialogBodyView) -> Self {
        Self
    }
}

impl ddd::Layered for SystemHotkeysDialogBodyModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for SystemHotkeysDialogBodyModel {
    type View = SystemHotkeysDialogBodyView;
}
