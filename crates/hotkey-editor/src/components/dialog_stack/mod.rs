pub(crate) fn nested_picker_dialog_is_present() -> bool {
    web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| {
            document
                .query_selector(".key-picker-shell, .sys-key-picker-shell")
                .ok()
                .flatten()
        })
        .is_some()
}
