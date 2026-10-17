use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

// How far below the pager's visible area the last mounted card counts as near:
// not at all, it must be on screen. The window opens and re-centres with cards to
// spare below, so growing it mid-swipe is only the fallback for a flurry of
// swipes that outruns them. Growing it any earlier mounts a card during every
// snap, and that mount is the main-thread work that makes the snap stutter.
const NEAR_MARGIN: &str = "0px";

/// Calls back when the last mounted card comes into the pager's view, so
/// the window can grow below before a swipe runs out of cards. An
/// `IntersectionObserver` reports this off the scroll path, a couple of times per
/// swipe at most, instead of the pager measuring every card on every scroll
/// frame. The observer is disconnected when the watch is dropped.
pub(crate) struct LastCardWatch {
    observer: web_sys::IntersectionObserver,
    _on_near: Closure<dyn FnMut()>,
}

impl LastCardWatch {
    pub(crate) fn new(
        container: &web_sys::Element,
        on_near: impl FnMut() + 'static,
    ) -> Option<Self> {
        let boxed_callback: Box<dyn FnMut()> = Box::new(on_near);
        let closure = Closure::wrap(boxed_callback);
        let options = web_sys::IntersectionObserverInit::new();
        options.set_root(Some(container));
        options.set_root_margin(NEAR_MARGIN);
        let callback = closure.as_ref().unchecked_ref();
        let observer = web_sys::IntersectionObserver::new_with_options(callback, &options).ok()?;
        Some(Self {
            observer,
            _on_near: closure,
        })
    }

    /// Watches the pager's current last card instead of any earlier one. Called
    /// after every window change, since a change can mount a new last card.
    pub(crate) fn watch_last_card(&self, container: &web_sys::Element, card_selector: &str) {
        self.observer.disconnect();
        let Ok(nodes) = container.query_selector_all(card_selector) else {
            return;
        };
        let card_count = nodes.length();
        let Some(last_index) = card_count.checked_sub(1) else {
            return;
        };
        let Some(node) = nodes.item(last_index) else {
            return;
        };
        let Ok(card) = node.dyn_into::<web_sys::Element>() else {
            return;
        };
        self.observer.observe(&card);
    }
}

impl Drop for LastCardWatch {
    fn drop(&mut self) {
        self.observer.disconnect();
    }
}
