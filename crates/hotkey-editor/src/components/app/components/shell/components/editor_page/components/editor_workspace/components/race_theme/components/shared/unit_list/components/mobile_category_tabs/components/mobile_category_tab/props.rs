use super::view::MobileCategoryTabView;
use dioxus::prelude::*;
use warcraft_api::UnitKind;

/// One mobile category tab, identified by its unit kind. Whether it is the active
/// category and the signal it writes on tap are read from editor context, so the tab
/// needs neither as a prop. Its active accent colour comes from the theme container's
/// `--race-color`, so the tab needs no race of its own either.
#[derive(Props, Clone, PartialEq)]
pub struct MobileCategoryTabProps {
    pub kind: UnitKind,
}

impl From<&MobileCategoryTabView> for MobileCategoryTabProps {
    fn from(view: &MobileCategoryTabView) -> Self {
        let MobileCategoryTabView { kind } = view.clone();
        Self { kind }
    }
}

impl ddd::Props for MobileCategoryTabProps {
    type View = MobileCategoryTabView;
}
