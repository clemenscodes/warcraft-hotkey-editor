use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};

use crate::components::dialogs::dialog_header::DialogHeader;

const BTN_SECONDARY: &str = "inline-flex items-center justify-center px-14 py-6 \
    rounded-lg border border-warcraft-blue text-warcraft-text-secondary \
    font-friz-quadrata text-[2rem] transition-all duration-[120ms] whitespace-nowrap \
    bg-[rgba(20,40,70,0.7)] [text-shadow:1px_1px_0_rgba(0,0,0,0.6)] \
    hover:border-warcraft-gold hover:text-warcraft-gold \
    hover:[box-shadow:0_0_12px_rgba(255,206,99,0.25)]";

const BTN_PRIMARY: &str = "inline-flex items-center justify-center px-14 py-6 \
    border border-warcraft-gold rounded-lg text-warcraft-gold \
    font-friz-quadrata text-[2rem] cursor-pointer select-none whitespace-nowrap \
    transition-all duration-[120ms] \
    bg-[linear-gradient(180deg,#2a5085_0%,#1a3a5c_100%)] \
    [text-shadow:1px_1px_0_rgba(0,0,0,0.92)] \
    hover:bg-[linear-gradient(180deg,#356dac_0%,#1f4a72_100%)] \
    hover:[box-shadow:0_0_12px_rgba(255,206,99,0.4)]";

#[derive(Props, Clone, PartialEq)]
pub(crate) struct DownloadInfoDialogProps {
    pub(crate) open: Signal<bool>,
    pub(crate) on_confirm: EventHandler<()>,
}

#[component]
pub(crate) fn DownloadInfoDialog(props: DownloadInfoDialogProps) -> Element {
    let mut open = props.open;
    let on_confirm = props.on_confirm;
    let handle_open_change = move |is_open| open.set(is_open);
    let handle_close = move |_| open.set(false);
    let handle_cancel = move |_| open.set(false);
    let handle_download = move |_| {
        open.set(false);
        on_confirm.call(());
    };
    rsx! {
        DialogRoot {
            class: "dialog-overlay",
            open: open(),
            on_open_change: handle_open_change,
            DialogContent {
                class: "dialog-shell wc3-dialog download-info-dialog".to_string(),
                DialogHeader {
                    title: "Download CustomKeys.txt".to_string(),
                    on_close: handle_close,
                }
                div { class: "wc3-dialog-body flex flex-col",
                    div {
                        class: "flex flex-col items-center justify-center gap-10 flex-1 \
                                max-w-[70rem] mx-auto w-full",
                        p { class: "m-0 text-[2rem] max-w-[90rem] text-center leading-snug font-friz-quadrata uppercase tracking-[0.1em] text-[rgba(255,206,99,0.75)] [text-shadow:1px_1px_0_#000]",
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
                            class: BTN_SECONDARY,
                            r#type: "button",
                            onclick: handle_cancel,
                            "Cancel"
                        }
                        button {
                            class: BTN_PRIMARY,
                            r#type: "button",
                            onclick: handle_download,
                            "Download"
                        }
                    }
                }
            }
        }
    }
}
