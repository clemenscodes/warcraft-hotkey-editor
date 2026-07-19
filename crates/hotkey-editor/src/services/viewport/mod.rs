use dioxus::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

// Matches the touch bands (`mobile` `< 768px` plus `tablet` `768-1279px`, i.e.
// everything below the `laptop` band) declared in tailwind.css, so the runtime
// mount decision agrees with the CSS bands exactly.
const TOUCH_MEDIA_QUERY: &str = "(max-width: 1279.98px)";

struct TouchMediaListener {
    query_list: web_sys::MediaQueryList,
    change_closure: Closure<dyn FnMut(web_sys::MediaQueryListEvent)>,
}

impl Drop for TouchMediaListener {
    fn drop(&mut self) {
        let callback = self.change_closure.as_ref().unchecked_ref();
        let _ = self
            .query_list
            .remove_event_listener_with_callback("change", callback);
    }
}

// Whether the viewport is currently in a touch band (mobile or tablet). Reactive:
// the returned bool updates when the viewport crosses the breakpoint. The `change`
// event fires from the browser event loop (never during a Dioxus render/commit), so
// writing the signal from its callback is safe. Shared by every page that swaps a
// desktop layout for the mobile pager, so the editor, the collisions page and the
// resolve page all decide the same way.
pub(crate) fn use_is_touch_viewport() -> bool {
    let mut is_touch = use_signal(|| {
        web_sys::window()
            .and_then(|window| window.match_media(TOUCH_MEDIA_QUERY).ok().flatten())
            .map(|query_list| query_list.matches())
            .unwrap_or(false)
    });

    use_hook(|| {
        let window = web_sys::window()?;
        let query_list = window.match_media(TOUCH_MEDIA_QUERY).ok().flatten()?;
        let change_closure = Closure::<dyn FnMut(web_sys::MediaQueryListEvent)>::new(
            move |event: web_sys::MediaQueryListEvent| {
                let matches = event.matches();
                if *is_touch.peek() != matches {
                    is_touch.set(matches);
                }
            },
        );
        let callback = change_closure.as_ref().unchecked_ref();
        let _ = query_list.add_event_listener_with_callback("change", callback);
        let listener = TouchMediaListener {
            query_list,
            change_closure,
        };
        Some(Rc::new(listener))
    });

    *is_touch.read()
}
