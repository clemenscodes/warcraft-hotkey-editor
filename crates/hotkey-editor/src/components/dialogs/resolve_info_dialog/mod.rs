use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};

use crate::components::dialogs::dialog_header::DialogHeader;

const BTN_SECONDARY: &str = "inline-flex items-center justify-center px-14 py-6 \
    rounded-lg border border-warcraft-blue text-warcraft-text-secondary \
    font-friz-quadrata text-[2rem] transition-all duration-[120ms] whitespace-nowrap \
    bg-[rgba(20,40,70,0.7)] [text-shadow:1px_1px_0_rgba(0,0,0,0.6)] \
    hover:border-warcraft-gold hover:text-warcraft-gold \
    hover:[box-shadow:0_0_12px_rgba(255,206,99,0.25)] \
    disabled:opacity-40 disabled:cursor-not-allowed disabled:hover:border-warcraft-blue \
    disabled:hover:text-warcraft-text-secondary disabled:hover:[box-shadow:none]";

const BTN_PRIMARY: &str = "inline-flex items-center justify-center gap-4 px-14 py-6 \
    border border-warcraft-gold rounded-lg text-warcraft-gold \
    font-friz-quadrata text-[2rem] cursor-pointer select-none whitespace-nowrap \
    transition-all duration-[120ms] \
    bg-[linear-gradient(180deg,#2a5085_0%,#1a3a5c_100%)] \
    [text-shadow:1px_1px_0_rgba(0,0,0,0.92)] \
    hover:bg-[linear-gradient(180deg,#356dac_0%,#1f4a72_100%)] \
    hover:[box-shadow:0_0_12px_rgba(255,206,99,0.4)] \
    disabled:opacity-60 disabled:cursor-wait disabled:hover:bg-[linear-gradient(180deg,#2a5085_0%,#1a3a5c_100%)] \
    disabled:hover:[box-shadow:none]";

const SPINNER_SVG: &str = r##"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" aria-hidden="true"><circle cx="12" cy="12" r="10" opacity="0.25"/><path d="M22 12a10 10 0 0 0-10-10"/></svg>"##;

#[derive(Props, Clone, PartialEq)]
pub(crate) struct ResolveInfoDialogProps {
    pub(crate) open: Signal<bool>,
    pub(crate) is_running: Signal<bool>,
    pub(crate) on_apply: EventHandler<()>,
}

#[component]
pub(crate) fn ResolveInfoDialog(props: ResolveInfoDialogProps) -> Element {
    let mut open = props.open;
    let is_running = props.is_running;
    let on_apply = props.on_apply;
    let running_now = *is_running.read();
    let handle_open_change = move |is_open: bool| {
        if running_now && !is_open {
            return;
        }
        open.set(is_open);
    };
    let handle_close = move |_| {
        if running_now {
            return;
        }
        open.set(false);
    };
    let handle_cancel = move |_| {
        if running_now {
            return;
        }
        open.set(false);
    };
    let handle_apply = move |_| {
        if running_now {
            return;
        }
        on_apply.call(());
    };
    rsx! {
        DialogRoot {
            class: "dialog-overlay",
            open: open(),
            on_open_change: handle_open_change,
            DialogContent {
                class: "dialog-shell wc3-dialog resolve-info-dialog".to_string(),
                DialogHeader {
                    title: "Resolve Conflicts".to_string(),
                    on_close: handle_close,
                }
                div { class: "wc3-dialog-body flex flex-col",
                    div {
                        class: "flex flex-col items-center justify-center gap-8 flex-1 \
                                max-w-[70rem] mx-auto w-full",
                        p { class: "m-0 text-[2rem] max-w-[90rem] text-center leading-snug font-friz-quadrata uppercase tracking-[0.1em] text-[rgba(255,206,99,0.85)] [text-shadow:1px_1px_0_#000]",
                            "Redistribute ability positions to remove cross-unit collisions."
                        }
                        ul {
                            class: "list-none m-0 p-0 w-full max-w-[90rem] flex flex-col gap-3 \
                                    text-[1.75rem] font-friz-quadrata uppercase tracking-[0.08em] \
                                    text-[rgba(255,206,99,0.7)] leading-snug [text-shadow:1px_1px_0_#000]",
                            li { "Hotkeys are preserved — only button positions change." }
                            li { "Same row first; cross-row only as a last resort." }
                            li { "Heavy commands like Attack, Hold, and Cancel never move." }
                            li { "Writes directly to your file. To revert, re-upload the original." }
                        }
                        if running_now {
                            div {
                                class: "flex items-center justify-center gap-5 mt-2 \
                                        font-friz-quadrata uppercase tracking-[0.1em] \
                                        text-[1.75rem] text-warcraft-gold [text-shadow:1px_1px_0_#000]",
                                role: "status",
                                aria_live: "polite",
                                span {
                                    class: "inline-flex w-[2rem] h-[2rem] text-warcraft-gold animate-spin",
                                    aria_hidden: "true",
                                    dangerous_inner_html: SPINNER_SVG,
                                }
                                "Resolving…"
                            }
                        }
                    }
                    div { class: "flex flex-wrap gap-4 justify-end flex-none pt-4",
                        button {
                            class: BTN_SECONDARY,
                            r#type: "button",
                            disabled: running_now,
                            onclick: handle_cancel,
                            "Cancel"
                        }
                        button {
                            class: BTN_PRIMARY,
                            r#type: "button",
                            disabled: running_now,
                            onclick: handle_apply,
                            if running_now {
                                span {
                                    class: "inline-flex w-[1.75rem] h-[1.75rem] animate-spin",
                                    aria_hidden: "true",
                                    dangerous_inner_html: SPINNER_SVG,
                                }
                                "Applying"
                            } else {
                                "Apply"
                            }
                        }
                    }
                }
            }
        }
    }
}
