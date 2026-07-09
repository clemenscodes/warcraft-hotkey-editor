use dioxus::prelude::*;
use warcraft_api::UnitKind;

/// One mobile category tab, identified by its unit kind. Whether it is the active
/// category and the signal it writes on tap are read from editor context, so the tab
/// needs neither as a prop. Its active accent colour comes from the theme container's
/// `--race-accent`, so the tab needs no race of its own either.
#[derive(Props, Clone, PartialEq)]
pub struct MobileCategoryTabProps {
    pub kind: UnitKind,
}
