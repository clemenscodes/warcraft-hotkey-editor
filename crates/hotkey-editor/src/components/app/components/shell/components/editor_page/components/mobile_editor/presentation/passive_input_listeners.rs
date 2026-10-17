use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

/// Native `passive` listeners on the pager for the input that starts a scroll.
/// Dioxus delegates its handlers to the app root as non-passive listeners, and a
/// non-passive `touchstart` or `wheel` makes the browser wait for the main
/// thread before it may scroll at all, so every swipe that starts while the main
/// thread is busy stutters. These listeners only observe the input (they never
/// cancel it), so the browser scrolls at once. They are removed when dropped.
pub(super) struct PassiveInputListeners {
    target: web_sys::EventTarget,
    registrations: Vec<ListenerRegistration>,
}

struct ListenerRegistration {
    event_type: &'static str,
    callback: Closure<dyn FnMut()>,
}

impl PassiveInputListeners {
    pub(super) fn new(target: &web_sys::Element) -> Self {
        let event_target: &web_sys::EventTarget = target.as_ref();
        Self {
            target: event_target.clone(),
            registrations: Vec::new(),
        }
    }

    /// Calls `handler` on every `event_type` event, as a passive listener.
    pub(super) fn listen(&mut self, event_type: &'static str, handler: impl FnMut() + 'static) {
        let boxed_handler: Box<dyn FnMut()> = Box::new(handler);
        let callback = Closure::wrap(boxed_handler);
        let options = web_sys::AddEventListenerOptions::new();
        options.set_passive(true);
        let function = callback.as_ref().unchecked_ref();
        let added = self
            .target
            .add_event_listener_with_callback_and_add_event_listener_options(
                event_type, function, &options,
            );
        if added.is_err() {
            return;
        }
        let registration = ListenerRegistration {
            event_type,
            callback,
        };
        self.registrations.push(registration);
    }
}

impl Drop for PassiveInputListeners {
    fn drop(&mut self) {
        for registration in &self.registrations {
            let function = registration.callback.as_ref().unchecked_ref();
            let _ = self
                .target
                .remove_event_listener_with_callback(registration.event_type, function);
        }
    }
}
