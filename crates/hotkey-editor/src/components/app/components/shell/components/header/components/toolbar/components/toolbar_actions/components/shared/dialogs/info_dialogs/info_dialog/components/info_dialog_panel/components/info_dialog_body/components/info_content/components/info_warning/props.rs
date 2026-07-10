use dioxus::prelude::*;

/// The warning callout's copy. `None` for dialogs without a warning, where nothing
/// renders.
#[derive(Props, Clone, PartialEq)]
pub struct InfoWarningProps {
    pub warning: Option<&'static str>,
}
