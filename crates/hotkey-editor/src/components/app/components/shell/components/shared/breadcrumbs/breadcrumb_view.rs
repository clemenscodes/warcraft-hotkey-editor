use dioxus::prelude::*;

/// One breadcrumb tab's data: its label, live count, active flag, and the
/// navigation handler it runs when clicked. A page builds a `Vec<BreadcrumbView>`
/// and hands it to the shared bar, which builds each tab's private props from these
/// named fields. This is the shared data contract; the tab's `Props` stay private.
#[derive(Clone, PartialEq)]
pub struct BreadcrumbView {
    pub label: String,
    pub count: usize,
    pub active: bool,
    pub onclick: EventHandler<MouseEvent>,
}
