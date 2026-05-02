use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};

use crate::components::dialog_header::DialogHeader;
use crate::customkeys::upload_picker::UploadPicker;

#[component]
pub(crate) fn UploadInfoDialog(mut open: Signal<bool>) -> Element {
    rsx! {
        DialogRoot {
            class: "dialog-overlay",
            open: open(),
            on_open_change: move |is_open| open.set(is_open),
            DialogContent {
                class: "dialog-shell wc3-dialog upload-info-dialog".to_string(),
                DialogHeader {
                    title: "Import CustomKeys.txt".to_string(),
                    on_close: move |_| open.set(false),
                }
                div { class: "wc3-dialog-body flex flex-col",
                    div {
                        class: "flex flex-col items-center justify-center gap-8 flex-1 \
                                max-w-[70rem] mx-auto w-full",
                        p { class: "wc3-stage-hint",
                            "Open your Documents folder, navigate to Warcraft III, \
                            then CustomKeyBindings, and select this file:"
                        }
                        div {
                            class: "font-mono text-[2rem] px-8 py-4 rounded-md \
                                    border border-[rgba(255,206,99,0.35)] \
                                    bg-[rgba(8,18,35,0.85)] text-warcraft-gold",
                            "CustomKeys.txt"
                        }
                    }
                    div { class: "flex flex-wrap gap-4 justify-end flex-none pt-4",
                        button {
                            class: "btn-warcraft-secondary",
                            r#type: "button",
                            onclick: move |_| open.set(false),
                            "Cancel"
                        }
                        button {
                            class: "btn-warcraft-primary",
                            r#type: "button",
                            onclick: move |_| {
                                open.set(false);
                                UploadPicker::trigger();
                            },
                            "Choose File"
                        }
                    }
                }
            }
        }
    }
}
