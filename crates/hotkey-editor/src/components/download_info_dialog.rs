use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};

use crate::components::dialog_header::DialogHeader;

#[component]
pub(crate) fn DownloadInfoDialog(mut open: Signal<bool>, on_confirm: EventHandler<()>) -> Element {
    rsx! {
        DialogRoot {
            class: "dialog-overlay",
            open: open(),
            on_open_change: move |is_open| open.set(is_open),
            DialogContent {
                class: "dialog-shell wc3-dialog download-info-dialog".to_string(),
                DialogHeader {
                    title: "Download CustomKeys.txt".to_string(),
                    on_close: move |_| open.set(false),
                }
                div { class: "wc3-dialog-body flex flex-col",
                    div {
                        class: "flex flex-col items-center justify-center gap-10 flex-1 \
                                max-w-[70rem] mx-auto w-full",
                        p { class: "wc3-stage-hint",
                            "Place the file in your Documents folder, inside Warcraft III, \
                            then CustomKeyBindings. The filename must be exactly:"
                        }
                        div {
                            class: "font-mono text-[2rem] px-8 py-4 rounded-md \
                                    border border-[rgba(255,206,99,0.35)] \
                                    bg-[rgba(8,18,35,0.85)] text-warcraft-gold",
                            "CustomKeys.txt"
                        }
                        p {
                            class: "w-full rounded-md px-6 py-5 m-0 text-center \
                                    font-friz-quadrata uppercase tracking-[0.08em] \
                                    text-[1.75rem] leading-relaxed",
                            style: "border: 1px solid rgba(255,180,0,0.45); \
                                    background: rgba(60,40,0,0.45); \
                                    color: rgba(255,206,99,0.85); \
                                    text-shadow: 1px 1px 0 #000;",
                            "Any other filename will not be detected by Warcraft III. \
                            Note: button positions in saved custom games are fixed at \
                            save time and will not update, even if hotkeys change."
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
                                on_confirm.call(());
                            },
                            "Download"
                        }
                    }
                }
            }
        }
    }
}
