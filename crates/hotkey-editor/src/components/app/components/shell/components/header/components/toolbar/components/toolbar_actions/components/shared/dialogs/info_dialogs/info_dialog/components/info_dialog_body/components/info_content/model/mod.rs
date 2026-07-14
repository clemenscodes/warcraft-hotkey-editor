use super::view::InfoContentView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InfoContentModel {
    pub intro: &'static str,
    pub warning: Option<&'static str>,
}

impl From<&InfoContentView> for InfoContentModel {
    fn from(view: &InfoContentView) -> Self {
        let InfoContentView { intro, warning } = view.clone();
        Self { intro, warning }
    }
}

impl ddd::Model for InfoContentModel {
    type View = InfoContentView;
}
