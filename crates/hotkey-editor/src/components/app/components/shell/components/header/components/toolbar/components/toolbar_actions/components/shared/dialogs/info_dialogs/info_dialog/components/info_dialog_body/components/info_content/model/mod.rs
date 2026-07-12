use super::view::InfoContentView;
use dioxus::prelude::*;

/// The instruction block's content: the intro line and the optional warning. The
/// filename chip is fixed, so it is not carried here.
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
