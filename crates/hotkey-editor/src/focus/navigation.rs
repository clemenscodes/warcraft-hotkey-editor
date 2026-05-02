use wasm_bindgen::{JsCast, JsValue};

pub(crate) struct FocusedElementInfo {
    classes: String,
    is_inside_grid_panel: bool,
    is_inside_system_dialog: bool,
}

impl FocusedElementInfo {
    pub(crate) fn current() -> Option<Self> {
        let active_element = web_sys::window()?.document()?.active_element()?;
        let classes = active_element.class_name();
        let is_inside_grid_panel = classes.contains("grid-tile")
            || classes.contains("override-key-cell")
            || classes.contains("tile-override-tier-button");
        let is_inside_system_dialog = active_element
            .closest(".system-hotkeys-dialog")
            .ok()
            .flatten()
            .is_some();
        let info = Self {
            classes,
            is_inside_grid_panel,
            is_inside_system_dialog,
        };
        Some(info)
    }

    pub(crate) fn classes(&self) -> &str {
        &self.classes
    }

    pub(crate) fn is_inside_grid_panel(&self) -> bool {
        self.is_inside_grid_panel
    }

    pub(crate) fn is_inside_system_dialog(&self) -> bool {
        self.is_inside_system_dialog
    }
}

pub(crate) struct FocusNavigation;

impl FocusNavigation {
    pub(crate) fn first_matching(selectors: &[&str]) -> bool {
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

    pub(crate) fn cycle_inside_unit_detail(reverse: bool) {
        Self::cycle_within_container(
            ".unit-detail",
            ".grid-tile.has-ability, .override-key-cell, .tile-override-tier-button",
            reverse,
        );
    }

    pub(crate) fn cycle_inside_system_dialog(reverse: bool) {
        Self::cycle_within_container(
            ".system-hotkeys-dialog",
            ".close-button, .system-key-cell",
            reverse,
        );
    }

    fn cycle_within_container(container_selector: &str, focusable_selectors: &str, reverse: bool) {
        let Some(document) = web_sys::window().and_then(|window| window.document()) else {
            return;
        };
        let Ok(Some(panel)) = document.query_selector(container_selector) else {
            return;
        };
        let Ok(node_list) = panel.query_selector_all(focusable_selectors) else {
            return;
        };
        let length_count = usize::try_from(node_list.length()).unwrap_or(0);
        let mut focusable_elements: Vec<web_sys::HtmlElement> = Vec::with_capacity(length_count);
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
        let next_index = match (current_index, reverse) {
            (None, _) => 0,
            (Some(index), false) => {
                if index + 1 >= focusable_elements.len() {
                    0
                } else {
                    index + 1
                }
            }
            (Some(index), true) => {
                if index == 0 {
                    focusable_elements.len() - 1
                } else {
                    index - 1
                }
            }
        };
        let _ = focusable_elements[next_index].focus();
    }
}
