use crate::components::shell::toasts::ToastRecord;
use crate::components::shell::toasts::hooks::ToastProviderModel;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastContainerProps {
    pub toasts: Vec<ToastRecord>,
    pub on_remove: Callback<usize>,
}

impl From<&ToastProviderModel> for ToastContainerProps {
    fn from(model: &ToastProviderModel) -> Self {
        let toasts = model.records();
        let on_remove = model.on_remove();
        Self { toasts, on_remove }
    }
}
