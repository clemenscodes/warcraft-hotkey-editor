use super::view::InfoDialogBodyView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InfoDialogBodyModel {
    pub intro: &'static str,
    pub warning: Option<&'static str>,
    pub primary_label: &'static str,
    pub on_primary: EventHandler<MouseEvent>,
    pub on_cancel: EventHandler<MouseEvent>,
}

impl From<&InfoDialogBodyView> for InfoDialogBodyModel {
    fn from(view: &InfoDialogBodyView) -> Self {
        let InfoDialogBodyView {
            intro,
            warning,
            primary_label,
            on_primary,
            on_cancel,
        } = view.clone();
        Self {
            intro,
            warning,
            primary_label,
            on_primary,
            on_cancel,
        }
    }
}

impl ddd::Model for InfoDialogBodyModel {
    type View = InfoDialogBodyView;
}
