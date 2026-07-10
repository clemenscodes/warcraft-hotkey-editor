use dioxus::prelude::*;

/// The published `View` contract mirroring [`InfoDialogPanelProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct InfoDialogPanelView {
    pub title: &'static str,
    pub on_close: EventHandler<()>,
    pub intro: &'static str,
    pub warning: Option<&'static str>,
    pub primary_label: &'static str,
    pub on_primary: EventHandler<MouseEvent>,
    pub on_cancel: EventHandler<MouseEvent>,
}

impl ddd::View for InfoDialogPanelView {}
