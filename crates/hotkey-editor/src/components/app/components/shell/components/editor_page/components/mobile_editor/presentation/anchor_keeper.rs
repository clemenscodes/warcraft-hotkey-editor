use super::super::state::PagerAnchor;
use super::super::state::PagerWindow;
use super::pager_dom::PagerDom;
use dioxus::prelude::*;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

/// Holds the card the pager is keeping in place and puts it back whenever the
/// layout moves it. An anchor stays held after it is first applied, because the
/// cards around it keep changing height after a render (the layout settling,
/// cards changing size); every such change re-applies it. It is released the
/// moment the user takes the scroll over (a finger, a wheel, a pointer, a key),
/// and the next settle holds a new one.
#[derive(Clone)]
pub(super) struct AnchorKeeper {
    element_ref: Rc<RefCell<Option<web_sys::Element>>>,
    window: Signal<PagerWindow>,
    held_anchor: Rc<Cell<Option<PagerAnchor>>>,
}

impl AnchorKeeper {
    pub(super) fn new(
        element_ref: Rc<RefCell<Option<web_sys::Element>>>,
        window: Signal<PagerWindow>,
    ) -> Self {
        let held_anchor = Rc::new(Cell::new(None));
        Self {
            element_ref,
            window,
            held_anchor,
        }
    }

    /// Starts holding `anchor` and applies it now. Returns whether the anchored
    /// card was in the DOM, so a caller can retry once it is.
    pub(super) fn hold(&self, anchor: PagerAnchor) -> bool {
        self.held_anchor.set(Some(anchor));
        self.reapply()
    }

    pub(super) fn release(&self) {
        self.held_anchor.set(None);
    }

    /// Scrolls the held anchor's card back to its anchor offset, by exactly the
    /// distance it has drifted. Does nothing when no anchor is held.
    pub(super) fn reapply(&self) -> bool {
        let Some(anchor) = self.held_anchor.get() else {
            return false;
        };
        let borrowed = self.element_ref.borrow();
        let Some(element) = borrowed.as_ref() else {
            return false;
        };
        let current_window = *self.window.peek();
        if !current_window.contains(anchor.slot()) {
            return false;
        }
        let anchor_slot_offset = anchor.slot() - current_window.first_slot();
        let Ok(card_offset) = usize::try_from(anchor_slot_offset) else {
            return false;
        };
        let pager_dom = PagerDom::new(element);
        let Some(card_offset_from_top) = pager_dom.card_offset_from_top(card_offset) else {
            return false;
        };
        let correction = card_offset_from_top - anchor.offset_from_top();
        pager_dom.scroll_by(correction);
        true
    }
}
