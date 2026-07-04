use dioxus::prelude::*;

/// The app subtree the toast provider wraps. Its `PartialEq` always reports
/// inequality so the provider re-renders on every parent update, keeping live
/// toasts in sync.
#[derive(Props, Clone)]
pub struct ToastMountProps {
    pub children: Element,
}

impl PartialEq for ToastMountProps {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
