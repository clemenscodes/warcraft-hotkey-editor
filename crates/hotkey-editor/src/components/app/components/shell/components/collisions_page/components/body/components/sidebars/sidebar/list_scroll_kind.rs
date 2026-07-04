use dioxus::prelude::*;

/// The kind of list scroll a [`super::Sidebar`] wraps its cards in. A zero-sized
/// marker whose impl renders the concrete scroll component, so the base sidebar
/// stays agnostic to which scroll it hosts — each kind extension binds its own.
pub trait ListScrollKind: Clone + PartialEq + Default + 'static {
    fn scroll(children: Element) -> Element;
}
