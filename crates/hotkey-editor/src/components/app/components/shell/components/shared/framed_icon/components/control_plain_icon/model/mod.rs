use super::view::ControlPlainIconView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ControlPlainIconModel {
    pub source: Option<String>,
    #[props(into)]
    pub alt: String,
}

impl From<&ControlPlainIconView> for ControlPlainIconModel {
    fn from(view: &ControlPlainIconView) -> Self {
        let ControlPlainIconView { source, alt } = view.clone();
        Self { source, alt }
    }
}

impl ddd::Model for ControlPlainIconModel {
    type View = ControlPlainIconView;
}
