use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

/// Native `passive` listeners on the pager for its scrolling and the touches
/// that drive it. They run straight on the element, without Dioxus's event
/// dispatch, because a scroll reports an event every frame. They only observe
/// the events (they never cancel them), so the browser's own scrolling is never
/// held up. They are removed when dropped.
pub(crate) struct PassiveInputListeners {
    target: web_sys::EventTarget,
    registrations: Vec<ListenerRegistration>,
}

struct ListenerRegistration {
    event_type: &'static str,
    callback: Closure<dyn FnMut(web_sys::Event)>,
}

impl PassiveInputListeners {
    pub(crate) fn new(target: &web_sys::Element) -> Self {
        let event_target: &web_sys::EventTarget = target.as_ref();
        Self {
            target: event_target.clone(),
            registrations: Vec::new(),
        }
    }

    /// Calls `handler` on every `event_type` event, as a passive listener.
    pub(crate) fn listen(
        &mut self,
        event_type: &'static str,
        handler: impl FnMut(web_sys::Event) + 'static,
    ) {
        let boxed_handler: Box<dyn FnMut(web_sys::Event)> = Box::new(handler);
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
