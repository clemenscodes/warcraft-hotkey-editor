use dioxus::document;

pub(crate) struct FocusModality;

impl FocusModality {
    pub(crate) fn after_render(selector: &str) {
        let escaped_selector = selector.replace('\\', "\\\\").replace('"', "\\\"");
        let script_source = format!(
            r#"if (window.__focusAfterRender) {{ window.__focusAfterRender("{escaped_selector}"); }}"#
        );
        document::eval(&script_source);
    }
}
