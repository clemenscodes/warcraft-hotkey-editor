use super::view::InfoDialogBodyView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InfoDialogBodyModel {
    pub intro: &'static str,
    pub warning: Option<&'static str>,
}

impl From<&InfoDialogBodyView> for InfoDialogBodyModel {
    fn from(view: &InfoDialogBodyView) -> Self {
        let InfoDialogBodyView { intro, warning } = view.clone();
        Self { intro, warning }
    }
}

impl ddd::Layered for InfoDialogBodyModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for InfoDialogBodyModel {
    type View = InfoDialogBodyView;
}
