use dioxus::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

// Matches the `mobile` responsive band (`< 768px`) declared in tailwind.css, so
// the runtime mount decision agrees with the CSS band exactly.
const MOBILE_MEDIA_QUERY: &str = "(max-width: 767.98px)";

struct MobileMediaListener {
    query_list: web_sys::MediaQueryList,
    change_closure: Closure<dyn FnMut(web_sys::MediaQueryListEvent)>,
}

impl Drop for MobileMediaListener {
    fn drop(&mut self) {
        let callback = self.change_closure.as_ref().unchecked_ref();
        let _ = self
            .query_list
            .remove_event_listener_with_callback("change", callback);
    }
}

// Whether the viewport is currently in the `mobile` band. Reactive: the returned
// bool updates when the viewport crosses the breakpoint. The `change` event fires
// from the browser event loop (never during a Dioxus render/commit), so writing
// the signal from its callback is safe.
pub(super) fn use_is_mobile_viewport() -> bool {
    let mut is_mobile = use_signal(|| {
        web_sys::window()
            .and_then(|window| window.match_media(MOBILE_MEDIA_QUERY).ok().flatten())
            .map(|query_list| query_list.matches())
            .unwrap_or(false)
    });

    use_hook(|| {
        let window = web_sys::window()?;
        let query_list = window.match_media(MOBILE_MEDIA_QUERY).ok().flatten()?;
        let change_closure = Closure::<dyn FnMut(web_sys::MediaQueryListEvent)>::new(
            move |event: web_sys::MediaQueryListEvent| {
                let matches = event.matches();
                if *is_mobile.peek() != matches {
                    is_mobile.set(matches);
                }
            },
        );
        let callback = change_closure.as_ref().unchecked_ref();
        let _ = query_list.add_event_listener_with_callback("change", callback);
        let listener = MobileMediaListener {
            query_list,
            change_closure,
        };
        Some(Rc::new(listener))
    });

    *is_mobile.read()
}
