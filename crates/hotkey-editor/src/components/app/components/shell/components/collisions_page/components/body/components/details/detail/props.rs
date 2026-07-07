use dioxus::prelude::*;

/// The shaped body of a loaded detail pane: the header content and the conflict
/// cards, already built by the extension.
#[derive(Clone, PartialEq)]
pub struct DetailBody {
    header: Element,
    cards: Element,
}

impl DetailBody {
    pub fn new(header: Element, cards: Element) -> Self {
        Self { header, cards }
    }

    pub fn header(&self) -> &Element {
        &self.header
    }

    pub fn cards(&self) -> &Element {
        &self.cards
    }
}

/// A detail pane's content: the empty prompt before a selection, or the loaded
/// header + cards. The kind extension builds this; the base only places it.
#[derive(Clone, PartialEq)]
pub enum DetailContent {
    Empty(&'static str),
    Loaded(DetailBody),
}

#[derive(Props, Clone, PartialEq)]
pub struct DetailProps {
    pub content: DetailContent,
}
