/// Keyboard focus movement within the DOM: focus the first match of a selector
/// list, or cycle focus among the focusable elements inside a container (a
/// roving-tabindex / focus-trap primitive). Both are pure DOM traversal; the
/// caller supplies the selectors, so nothing here is app-specific.
pub struct RovingFocus;

impl RovingFocus {
    /// Focus the first element matching any selector in order. Returns whether
    /// one was focused.
    pub fn first_matching(selectors: &[&str]) -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            let Some(document) = web_sys::window().and_then(|window| window.document()) else {
                return false;
            };
            for selector in selectors {
                if let Ok(Some(element)) = document.query_selector(selector)
                    && let Ok(html_element) = element.dyn_into::<web_sys::HtmlElement>()
                {
                    let _ = html_element.focus();
                    return true;
                }
            }
            false
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = selectors;
            false
        }
    }

    /// Move focus to the next (or previous, when `reverse`) focusable element
    /// matching `focusable_selectors` inside the first `container_selector`.
    /// Elements with a negative tab index are skipped; focus wraps around; if
    /// nothing is currently focused inside, the first element is taken.
    pub fn cycle(container_selector: &str, focusable_selectors: &str, reverse: bool) {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::{JsCast, JsValue};
            let Some(document) = web_sys::window().and_then(|window| window.document()) else {
                return;
            };
            let Ok(Some(container)) = document.query_selector(container_selector) else {
                return;
            };
            let Ok(node_list) = container.query_selector_all(focusable_selectors) else {
                return;
            };
            let length_count = usize::try_from(node_list.length()).unwrap_or(0);
            let mut focusable_elements: Vec<web_sys::HtmlElement> =
                Vec::with_capacity(length_count);
            for index in 0..node_list.length() {
                let Some(node) = node_list.item(index) else {
                    continue;
                };
                let Ok(html_element) = node.dyn_into::<web_sys::HtmlElement>() else {
                    continue;
                };
                if html_element.tab_index() < 0 {
                    continue;
                }
                focusable_elements.push(html_element);
            }
            if focusable_elements.is_empty() {
                return;
            }
            let active_element = document.active_element();
            let active_value: Option<JsValue> = active_element
                .as_ref()
                .map(|element| element.clone().into());
            let current_index = active_value.as_ref().and_then(|active_js| {
                focusable_elements.iter().position(|focusable_element| {
                    <web_sys::HtmlElement as AsRef<JsValue>>::as_ref(focusable_element) == active_js
                })
            });
            let last_index = focusable_elements.len() - 1;
            let next_index = match (current_index, reverse) {
                (None, _) => 0,
                (Some(index), false) => {
                    if index >= last_index {
                        0
                    } else {
                        index + 1
                    }
                }
                (Some(index), true) => {
                    if index == 0 {
                        last_index
                    } else {
                        index - 1
                    }
                }
            };
            let _ = focusable_elements[next_index].focus();
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (container_selector, focusable_selectors, reverse);
        }
    }
}
