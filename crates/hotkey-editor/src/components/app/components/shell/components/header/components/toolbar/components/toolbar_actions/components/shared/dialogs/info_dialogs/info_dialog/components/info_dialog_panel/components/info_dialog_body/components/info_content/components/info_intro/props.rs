use super::view::InfoIntroView;
use dioxus::prelude::*;

/// The intro line's copy.
#[derive(Props, Clone, PartialEq)]
pub struct InfoIntroProps {
    pub intro: &'static str,
}

impl From<&InfoIntroView> for InfoIntroProps {
    fn from(view: &InfoIntroView) -> Self {
        let InfoIntroView { intro } = view.clone();
        Self { intro }
    }
}

impl ddd::Props for InfoIntroProps {
    type View = InfoIntroView;
}
