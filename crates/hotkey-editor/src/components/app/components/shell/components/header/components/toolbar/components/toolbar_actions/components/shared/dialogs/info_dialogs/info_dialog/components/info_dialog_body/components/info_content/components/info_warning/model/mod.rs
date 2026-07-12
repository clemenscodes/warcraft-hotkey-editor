use super::view::InfoWarningView;
use dioxus::prelude::*;

/// The warning callout's copy. `None` for dialogs without a warning, where nothing
/// renders.
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
