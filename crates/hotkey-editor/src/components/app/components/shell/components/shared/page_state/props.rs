use dioxus::prelude::*;

/// The typed leaves a page hands its centered state section: the all-clear glyph
/// and label, or the upload-prompt message. `PageState` is a structural shell —
/// it owns the centered layout and nothing page-specific — so its content arrives
/// as children rather than a fixed prop set.
#[derive(Props, Clone, PartialEq)]
pub struct PageStateProps {
    pub children: Element,
}
