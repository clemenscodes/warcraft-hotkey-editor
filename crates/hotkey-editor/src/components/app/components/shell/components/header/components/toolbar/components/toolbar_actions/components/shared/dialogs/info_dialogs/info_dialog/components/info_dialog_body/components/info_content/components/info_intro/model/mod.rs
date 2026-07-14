use super::view::InfoIntroView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InfoIntroModel {
    pub intro: &'static str,
}

impl From<&InfoIntroView> for InfoIntroModel {
    fn from(view: &InfoIntroView) -> Self {
        let InfoIntroView { intro } = view.clone();
        Self { intro }
    }
}

impl ddd::Model for InfoIntroModel {
    type View = InfoIntroView;
}
