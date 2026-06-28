mod browser_event;

use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};
use warcraft_keybinds::{Digit, FunctionKey, KeyCode, Letter, MouseButton, NumpadKey, Punctuation};

use crate::components::dialogs::dialog_header::DialogHeader;
use browser_event::BrowserKeyEvent;

/// One key on the on-screen board: the domain key it stands for plus the compact
/// label shown on its cap. The label is deliberately shorter than the key's full
/// `Display` (for example `Ins` rather than `Insert`) to fit the keyboard layout.
struct BoardKey {
    code: KeyCode,
    label: &'static str,
}

static KEYBOARD_ROWS: &[&[BoardKey]] = &[
    &[
        BoardKey {
            code: KeyCode::Escape,
            label: "Esc",
        },
        BoardKey {
            code: KeyCode::Function(FunctionKey::F1),
            label: "F1",
        },
        BoardKey {
            code: KeyCode::Function(FunctionKey::F2),
            label: "F2",
        },
        BoardKey {
            code: KeyCode::Function(FunctionKey::F3),
            label: "F3",
        },
        BoardKey {
            code: KeyCode::Function(FunctionKey::F4),
            label: "F4",
        },
        BoardKey {
            code: KeyCode::Function(FunctionKey::F5),
            label: "F5",
        },
        BoardKey {
            code: KeyCode::Function(FunctionKey::F6),
            label: "F6",
        },
        BoardKey {
            code: KeyCode::Function(FunctionKey::F7),
            label: "F7",
        },
        BoardKey {
            code: KeyCode::Function(FunctionKey::F8),
            label: "F8",
        },
        BoardKey {
            code: KeyCode::Function(FunctionKey::F9),
            label: "F9",
        },
        BoardKey {
            code: KeyCode::Function(FunctionKey::F10),
            label: "F10",
        },
        BoardKey {
            code: KeyCode::Function(FunctionKey::F11),
            label: "F11",
        },
        BoardKey {
            code: KeyCode::Function(FunctionKey::F12),
            label: "F12",
        },
    ],
    &[
        BoardKey {
            code: KeyCode::Punctuation(Punctuation::Backtick),
            label: "`",
        },
        BoardKey {
            code: KeyCode::Digit(Digit::One),
            label: "1",
        },
        BoardKey {
            code: KeyCode::Digit(Digit::Two),
            label: "2",
        },
        BoardKey {
            code: KeyCode::Digit(Digit::Three),
            label: "3",
        },
        BoardKey {
            code: KeyCode::Digit(Digit::Four),
            label: "4",
        },
        BoardKey {
            code: KeyCode::Digit(Digit::Five),
            label: "5",
        },
        BoardKey {
            code: KeyCode::Digit(Digit::Six),
            label: "6",
        },
        BoardKey {
            code: KeyCode::Digit(Digit::Seven),
            label: "7",
        },
        BoardKey {
            code: KeyCode::Digit(Digit::Eight),
            label: "8",
        },
        BoardKey {
            code: KeyCode::Digit(Digit::Nine),
            label: "9",
        },
        BoardKey {
            code: KeyCode::Digit(Digit::Zero),
            label: "0",
        },
        BoardKey {
            code: KeyCode::Punctuation(Punctuation::Minus),
            label: "-",
        },
        BoardKey {
            code: KeyCode::Punctuation(Punctuation::Equals),
            label: "=",
        },
    ],
    &[
        BoardKey {
            code: KeyCode::Letter(Letter::Q),
            label: "Q",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::W),
            label: "W",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::E),
            label: "E",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::R),
            label: "R",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::T),
            label: "T",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::Y),
            label: "Y",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::U),
            label: "U",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::I),
            label: "I",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::O),
            label: "O",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::P),
            label: "P",
        },
        BoardKey {
            code: KeyCode::Punctuation(Punctuation::LeftBracket),
            label: "[",
        },
        BoardKey {
            code: KeyCode::Punctuation(Punctuation::RightBracket),
            label: "]",
        },
        BoardKey {
            code: KeyCode::Punctuation(Punctuation::Backslash),
            label: "\\",
        },
    ],
    &[
        BoardKey {
            code: KeyCode::Letter(Letter::A),
            label: "A",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::S),
            label: "S",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::D),
            label: "D",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::F),
            label: "F",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::G),
            label: "G",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::H),
            label: "H",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::J),
            label: "J",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::K),
            label: "K",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::L),
            label: "L",
        },
        BoardKey {
            code: KeyCode::Punctuation(Punctuation::Semicolon),
            label: ";",
        },
        BoardKey {
            code: KeyCode::Punctuation(Punctuation::Quote),
            label: "'",
        },
    ],
    &[
        BoardKey {
            code: KeyCode::Letter(Letter::Z),
            label: "Z",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::X),
            label: "X",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::C),
            label: "C",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::V),
            label: "V",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::B),
            label: "B",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::N),
            label: "N",
        },
        BoardKey {
            code: KeyCode::Letter(Letter::M),
            label: "M",
        },
        BoardKey {
            code: KeyCode::Punctuation(Punctuation::Comma),
            label: ",",
        },
        BoardKey {
            code: KeyCode::Punctuation(Punctuation::Period),
            label: ".",
        },
        BoardKey {
            code: KeyCode::Punctuation(Punctuation::Slash),
            label: "/",
        },
    ],
    &[
        BoardKey {
            code: KeyCode::Space,
            label: "Space",
        },
        BoardKey {
            code: KeyCode::Backspace,
            label: "Backspace",
        },
        BoardKey {
            code: KeyCode::Insert,
            label: "Ins",
        },
        BoardKey {
            code: KeyCode::Delete,
            label: "Del",
        },
        BoardKey {
            code: KeyCode::Home,
            label: "Home",
        },
        BoardKey {
            code: KeyCode::End,
            label: "End",
        },
        BoardKey {
            code: KeyCode::PageUp,
            label: "PgUp",
        },
        BoardKey {
            code: KeyCode::PageDown,
            label: "PgDn",
        },
    ],
    &[
        BoardKey {
            code: KeyCode::Up,
            label: "↑",
        },
        BoardKey {
            code: KeyCode::Left,
            label: "←",
        },
        BoardKey {
            code: KeyCode::Down,
            label: "↓",
        },
        BoardKey {
            code: KeyCode::Right,
            label: "→",
        },
    ],
    // VK_XBUTTON1 (5, back) and VK_XBUTTON2 (6, forward) — confirmed against
    // a CustomKeys.txt exported by the in-game hotkey editor.
    &[
        BoardKey {
            code: KeyCode::Mouse(MouseButton::Back),
            label: "Mouse4",
        },
        BoardKey {
            code: KeyCode::Mouse(MouseButton::Forward),
            label: "Mouse5",
        },
    ],
];

static NUMPAD_ROWS: &[&[BoardKey]] = &[
    &[
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Num7),
            label: "Num7",
        },
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Num8),
            label: "Num8",
        },
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Num9),
            label: "Num9",
        },
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Divide),
            label: "Num/",
        },
    ],
    &[
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Num4),
            label: "Num4",
        },
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Num5),
            label: "Num5",
        },
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Num6),
            label: "Num6",
        },
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Multiply),
            label: "Num*",
        },
    ],
    &[
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Num1),
            label: "Num1",
        },
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Num2),
            label: "Num2",
        },
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Num3),
            label: "Num3",
        },
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Subtract),
            label: "Num-",
        },
    ],
    &[
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Num0),
            label: "Num0",
        },
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Decimal),
            label: "Num.",
        },
        BoardKey {
            code: KeyCode::Numpad(NumpadKey::Add),
            label: "Num+",
        },
    ],
];

#[derive(Props, Clone, PartialEq)]
pub struct SystemKeyPickerDialogProps {
    pub title: String,
    pub current_code: KeyCode,
    pub conflicts: HashMap<KeyCode, Vec<String>>,
    pub open: bool,
    pub on_pick: EventHandler<KeyCode>,
    pub on_close: EventHandler<()>,
}

#[component]
pub fn SystemKeyPickerDialog(props: SystemKeyPickerDialogProps) -> Element {
    let title = props.title;
    let current_code = props.current_code;
    let conflicts = props.conflicts;
    let open = props.open;
    let on_pick = props.on_pick;
    let on_close = props.on_close;
    let dialog_title = title.clone();
    // The keydown handler only fires while focus sits inside this dialog, but the
    // portal-mounted content has its focus reset to `document.body` a tick after
    // mount, so `autofocus` only ever worked on the first open. Defer past that
    // reset and focus the body element ourselves so every reopen stays keyboard-
    // ready. See the matching effect in the shared `KeyPicker`.
    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        spawn(async move {
            use wasm_bindgen::JsCast;
            gloo_timers::future::TimeoutFuture::new(0).await;
            let Some(document) = web_sys::window().and_then(|window| window.document()) else {
                return;
            };
            let Some(node) = document
                .query_selector(".sys-key-picker-body")
                .ok()
                .flatten()
            else {
                return;
            };
            if let Some(focusable) = node.dyn_ref::<web_sys::HtmlElement>() {
                let _ = focusable.focus();
            }
        });
    });
    let handle_open_change = move |next_open: bool| {
        if !next_open {
            on_close.call(());
        }
    };
    let handle_keydown = move |event: Event<KeyboardData>| {
        event.stop_propagation();
        let key_val = event.data().key().to_string();
        if key_val == "Escape" {
            event.prevent_default();
            on_close.call(());
            return;
        }
        let code_val = event.data().code().to_string();
        let browser_event = BrowserKeyEvent::new(&key_val, &code_val);
        let Some(code) = browser_event.key_code() else {
            return;
        };
        // Only accept keys the board actually offers (the same cells shown and
        // clickable). `BrowserKeyEvent` also maps Tab/Enter and similar keys the
        // game does not bind, so reject anything not on the board to keep
        // keyboard input in step with what the UI presents.
        let is_offered = KEYBOARD_ROWS
            .iter()
            .chain(NUMPAD_ROWS.iter())
            .flat_map(|row| row.iter())
            .any(|entry| entry.code == code);
        if !is_offered {
            return;
        }
        event.prevent_default();
        on_pick.call(code);
    };
    let handle_close = move |_| on_close.call(());
    rsx! {
        DialogRoot {
            class: "dialog-overlay",
            open,
            on_open_change: handle_open_change,
            DialogContent { class: "dialog-shell wc3-dialog sys-key-picker-shell".to_string(),
                div {
                    class: "dialog-key-scope",
                    onkeydown: handle_keydown,
                    DialogHeader {
                        title: dialog_title,
                        on_close: handle_close,
                    }
                    div {
                        class: "wc3-dialog-body sys-key-picker-body",
                        tabindex: "-1",
                        div { class: "sys-key-picker-board",
                            div { class: "sys-key-picker-main",
                                for (row_idx, row) in KEYBOARD_ROWS.iter().enumerate() {
                                    {
                                        let total_rows = KEYBOARD_ROWS.len();
                                        let is_bottom_row = row_idx + 2 >= total_rows;
                                        let placement_attribute = if is_bottom_row { "above" } else { "below" };
                                        let last_index = row.len().saturating_sub(1);
                                        rsx! {
                                            div { key: "{row_idx}", class: "sys-key-picker-row",
                                                for (key_idx, entry) in row.iter().enumerate() {
                                                    {
                                                        let code = entry.code;
                                                        let label = entry.label;
                                                        let is_current = code == current_code;
                                                        let is_wide = matches!(label, "Space" | "Mouse4" | "Mouse5" | "Backspace");
                                                        let conflict_names = conflicts.get(&code);
                                                        let is_conflict = conflict_names.is_some();
                                                        let cls = if is_current {
                                                            "sys-key-picker-key current"
                                                        } else if is_conflict {
                                                            "sys-key-picker-key conflict"
                                                        } else {
                                                            "sys-key-picker-key"
                                                        };
                                                        let title_attribute = conflict_names
                                                            .map(|names| format!("Already used by {}", names.join(", ")))
                                                            .unwrap_or_default();
                                                        let anchor_attribute = if key_idx == 0 {
                                                            "left"
                                                        } else if key_idx == last_index {
                                                            "right"
                                                        } else {
                                                            ""
                                                        };
                                                        let handle_click = move |_| on_pick.call(code);
                                                        rsx! {
                                                            button {
                                                                key: "{key_idx}",
                                                                class: cls,
                                                                r#type: "button",
                                                                "data-tooltip": title_attribute,
                                                                "data-tooltip-placement": placement_attribute,
                                                                "data-tooltip-anchor": anchor_attribute,
                                                                "data-wide": if is_wide { "true" } else { "" },
                                                                onclick: handle_click,
                                                                {label}
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            div { class: "sys-key-picker-numpad",
                                for (row_idx, row) in NUMPAD_ROWS.iter().enumerate() {
                                    {
                                        let total_rows = NUMPAD_ROWS.len();
                                        let is_bottom_row = row_idx + 2 >= total_rows;
                                        let placement_attribute = if is_bottom_row { "above" } else { "below" };
                                        rsx! {
                                            div { key: "{row_idx}", class: "sys-key-picker-row",
                                                for (key_idx, entry) in row.iter().enumerate() {
                                                    {
                                                        let code = entry.code;
                                                        let label = entry.label;
                                                        let is_current = code == current_code;
                                                        let conflict_names = conflicts.get(&code);
                                                        let is_conflict = conflict_names.is_some();
                                                        let cls = if is_current {
                                                            "sys-key-picker-key current"
                                                        } else if is_conflict {
                                                            "sys-key-picker-key conflict"
                                                        } else {
                                                            "sys-key-picker-key"
                                                        };
                                                        let title_attribute = conflict_names
                                                            .map(|names| format!("Already used by {}", names.join(", ")))
                                                            .unwrap_or_default();
                                                        let handle_click = move |_| on_pick.call(code);
                                                        rsx! {
                                                            button {
                                                                key: "{key_idx}",
                                                                class: cls,
                                                                r#type: "button",
                                                                "data-tooltip": title_attribute,
                                                                "data-tooltip-placement": placement_attribute,
                                                                "data-tooltip-anchor": "right",
                                                                onclick: handle_click,
                                                                {label}
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
