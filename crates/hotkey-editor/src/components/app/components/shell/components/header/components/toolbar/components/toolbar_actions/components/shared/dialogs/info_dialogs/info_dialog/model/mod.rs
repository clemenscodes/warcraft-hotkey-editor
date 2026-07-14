use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InfoDialogConfig {
    pub open: bool,
    pub on_open_change: Callback<bool>,
    pub title: &'static str,
    pub intro: &'static str,
    pub warning: Option<&'static str>,
    pub primary_label: &'static str,
    pub on_primary: EventHandler<MouseEvent>,
    pub on_cancel: EventHandler<MouseEvent>,
}
