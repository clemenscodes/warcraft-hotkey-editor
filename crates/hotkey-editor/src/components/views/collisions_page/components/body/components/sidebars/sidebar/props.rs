use super::list_scroll_kind::ListScrollKind;
use dioxus::prelude::*;

/// The base sidebar's props: the scroll-kind marker the extension binds, plus the
/// cards the extension feeds into the slot. The base owns the shell; the kind
/// decides the concrete scroll, and the children decide what fills it.
#[derive(Props, Clone, PartialEq)]
pub struct SidebarProps<B: ListScrollKind> {
    pub kind: B,
    pub children: Element,
}
