use super::view::SystemHotkeysDialogBodyView;

/// The system-hotkeys body's ddd model. The component sources its UI state from context in
/// its presentation builder, so this is the fieldless published contract the frame's body
/// region names as its `Render::Model`.
#[derive(Clone, PartialEq, Default)]
pub struct SystemHotkeysDialogBodyModel;

impl From<&SystemHotkeysDialogBodyView> for SystemHotkeysDialogBodyModel {
    fn from(_view: &SystemHotkeysDialogBodyView) -> Self {
        Self
    }
}

impl ddd::Model for SystemHotkeysDialogBodyModel {
    type View = SystemHotkeysDialogBodyView;
}
