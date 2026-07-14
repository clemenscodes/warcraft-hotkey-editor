use super::view::EmptyStateView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct EmptyStateModel {
    #[props(into)]
    pub message: String,
}

impl From<&EmptyStateView> for EmptyStateModel {
    fn from(view: &EmptyStateView) -> Self {
        let EmptyStateView { message } = view.clone();
        Self { message }
    }
}

impl ddd::Model for EmptyStateModel {
    type View = EmptyStateView;
}
