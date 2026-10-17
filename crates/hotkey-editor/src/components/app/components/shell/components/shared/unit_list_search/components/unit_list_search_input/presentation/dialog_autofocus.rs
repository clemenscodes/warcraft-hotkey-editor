use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

// What marks the dialog the field sits in.
const DIALOG_SELECTOR: &str = "[role=\"dialog\"]";

/// Puts the cursor in the search field when its dialog opens. The dialog's focus
/// trap focuses its first focusable element, the close button, once it is set
/// up, which is after the field is mounted; so the field focuses itself and
/// takes the focus back the one time the trap moves it elsewhere. The listener
/// is removed when this is dropped.
pub(super) struct DialogAutofocus {
    dialog: web_sys::Element,
    on_focus_moved: Closure<dyn FnMut(web_sys::Event)>,
}

impl DialogAutofocus {
    pub(super) fn new(field: web_sys::HtmlElement) -> Option<Self> {
        let _ = field.focus();
        let dialog = field.closest(DIALOG_SELECTOR).ok()??;
        let refocused_field = field.clone();
        let on_focus_moved = move |event: web_sys::Event| {
            let field_target: &web_sys::EventTarget = refocused_field.as_ref();
            let moved_elsewhere = event.target().is_some_and(|target| &target != field_target);
            if moved_elsewhere {
                let _ = refocused_field.focus();
            }
        };
        let boxed_callback: Box<dyn FnMut(web_sys::Event)> = Box::new(on_focus_moved);
        let closure = Closure::wrap(boxed_callback);
        let options = web_sys::AddEventListenerOptions::new();
        options.set_once(true);
        let callback = closure.as_ref().unchecked_ref();
        dialog
            .add_event_listener_with_callback_and_add_event_listener_options(
                "focusin", callback, &options,
            )
            .ok()?;
        Some(Self {
            dialog,
            on_focus_moved: closure,
        })
    }
}

impl Drop for DialogAutofocus {
    fn drop(&mut self) {
        let callback = self.on_focus_moved.as_ref().unchecked_ref();
        let _ = self
            .dialog
            .remove_event_listener_with_callback("focusin", callback);
    }
}
