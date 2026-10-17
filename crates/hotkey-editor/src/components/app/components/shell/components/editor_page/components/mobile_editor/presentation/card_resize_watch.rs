use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

/// Calls back whenever a mounted card changes size: the layout settling after
/// the first render, an icon finishing loading, a viewport resize. The pager
/// re-applies its anchor from that callback, so the card it aligned stays put
/// while the cards around it grow or shrink. The observer is disconnected when
/// the watch is dropped.
pub(super) struct CardResizeWatch {
    observer: web_sys::ResizeObserver,
    _on_resize: Closure<dyn FnMut()>,
}

impl CardResizeWatch {
    pub(super) fn new(on_resize: impl FnMut() + 'static) -> Option<Self> {
        let boxed_callback: Box<dyn FnMut()> = Box::new(on_resize);
        let closure = Closure::wrap(boxed_callback);
        let callback = closure.as_ref().unchecked_ref();
        let observer = web_sys::ResizeObserver::new(callback).ok()?;
        Some(Self {
            observer,
            _on_resize: closure,
        })
    }

    /// Starts watching every card currently mounted in the pager. Watching a
    /// card twice is a no-op, so this is safe to call after every window change.
    pub(super) fn observe_cards(&self, container: &web_sys::Element) {
        let Ok(nodes) = container.query_selector_all(".pager-card-host") else {
            return;
        };
        for index in 0..nodes.length() {
            let Some(node) = nodes.item(index) else {
                continue;
            };
            let Ok(card) = node.dyn_into::<web_sys::Element>() else {
                continue;
            };
            self.observer.observe(&card);
        }
    }
}

impl Drop for CardResizeWatch {
    fn drop(&mut self) {
        self.observer.disconnect();
    }
}
