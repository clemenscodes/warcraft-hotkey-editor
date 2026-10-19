use dioxus::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

// Matches the pager bands (`mobile` `< 768px`, `tablet` `768-1279px` and `laptop`
// `1280-1919px`, i.e. everything below the `desktop` band) declared in
// tailwind.css, so the runtime mount decision agrees with the CSS bands exactly.
const PAGER_MEDIA_QUERY: &str = "(max-width: 1919.98px)";
// Matches the `laptop` band (`1280-1919px`), where the pager shows two units
// side by side on every page.
const LAPTOP_MEDIA_QUERY: &str = "(min-width: 1280px) and (max-width: 1919.98px)";

struct MediaQueryListener {
    query_list: web_sys::MediaQueryList,
    change_closure: Closure<dyn FnMut(web_sys::MediaQueryListEvent)>,
}

impl Drop for MediaQueryListener {
    fn drop(&mut self) {
        let callback = self.change_closure.as_ref().unchecked_ref();
        let _ = self
            .query_list
            .remove_event_listener_with_callback("change", callback);
    }
}

// Whether the viewport is currently in a pager band (mobile, tablet or laptop). Reactive:
// the returned bool updates when the viewport crosses the breakpoint. Shared by every
// page that swaps a desktop layout for the mobile pager, so the editor, the collisions
// page and the resolve page all decide the same way.
pub(crate) fn use_is_pager_viewport() -> bool {
    let is_pager = use_media_query(PAGER_MEDIA_QUERY);
    *is_pager.read()
}

// Whether the viewport is currently in the laptop band, as a signal so an effect
// that lays the pager out can rerun when the viewport crosses into or out of it.
pub(crate) fn use_laptop_viewport() -> Signal<bool> {
    use_media_query(LAPTOP_MEDIA_QUERY)
}

// Whether `query` matches right now, kept current by the query's `change` event. The
// event fires from the browser event loop (never during a Dioxus render/commit), so
// writing the signal from its callback is safe.
fn use_media_query(query: &'static str) -> Signal<bool> {
    let mut matches_signal = use_signal(|| {
        web_sys::window()
            .and_then(|window| window.match_media(query).ok().flatten())
            .map(|query_list| query_list.matches())
            .unwrap_or(false)
    });

    use_hook(|| {
        let window = web_sys::window()?;
        let query_list = window.match_media(query).ok().flatten()?;
        let change_closure = Closure::<dyn FnMut(web_sys::MediaQueryListEvent)>::new(
            move |event: web_sys::MediaQueryListEvent| {
                let matches = event.matches();
                if *matches_signal.peek() != matches {
                    matches_signal.set(matches);
                }
            },
        );
        let callback = change_closure.as_ref().unchecked_ref();
        let _ = query_list.add_event_listener_with_callback("change", callback);
        let listener = MediaQueryListener {
            query_list,
            change_closure,
        };
        Some(Rc::new(listener))
    });

    matches_signal
}
