use super::view::InfoWarningView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InfoWarningModel {
    pub warning: Option<&'static str>,
}

impl From<&InfoWarningView> for InfoWarningModel {
    fn from(view: &InfoWarningView) -> Self {
        let InfoWarningView { warning } = view.clone();
        Self { warning }
    }
}

impl ddd::Model for InfoWarningModel {
    type View = InfoWarningView;
}
