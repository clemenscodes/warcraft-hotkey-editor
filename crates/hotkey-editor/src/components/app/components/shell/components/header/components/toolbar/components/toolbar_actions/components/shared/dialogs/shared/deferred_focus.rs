use dioxus::prelude::*;

pub(crate) fn use_deferred_focus(selector: &'static str) {
    use_effect(move || {
        spawn(async move {
            use wasm_bindgen::JsCast;
            gloo_timers::future::TimeoutFuture::new(0).await;
            let Some(document) = web_sys::window().and_then(|window| window.document()) else {
                return;
            };
            let Some(node) = document.query_selector(selector).ok().flatten() else {
                return;
            };
            if let Some(focusable) = node.dyn_ref::<web_sys::HtmlElement>() {
                let _ = focusable.focus();
            }
        });
    });
}
