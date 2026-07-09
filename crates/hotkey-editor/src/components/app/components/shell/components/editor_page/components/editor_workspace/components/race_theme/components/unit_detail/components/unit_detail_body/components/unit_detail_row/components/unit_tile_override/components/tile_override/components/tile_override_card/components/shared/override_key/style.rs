use tw_macro::tw;
// The override key's host button: the focusable, keyboard-navigable box around the
// shared `EditableKeycap`. It owns only the *box* — the square size (and the widening
// for multi-character special tokens like Esc / Mouse4, via `data-special`), the font
// size the cap inherits, focus suppression, and the soft focus glow. The gold cap look
// and the capture pulse live on the nested `EditableKeycap`; the `group/editable-keycap`
// marker lets that cap reflect this button's keyboard focus. Class `.override-key` is
// load-bearing (keyboard navigation).

classes! {
    base: tw![
        "group/editable-keycap",
        "[--keycap-radius:var(--radius-tile)]",
        "flex",
        "items-center",
        "justify-center",
        "w-20",
        "h-20",
        "p-0",
        "text-2xl",
        "cursor-pointer",
        "kb-focus:outline-none",
        "kb-focus:shadow-glow-soft",
        "data-[special=true]:w-auto",
        "data-[special=true]:min-w-20",
        "data-[special=true]:text-xl",
        "data-[special=true]:whitespace-nowrap",
        "data-[special=true]:tracking-normal",
    ],
    mobile: tw![
        "mobile:w-[4.6rem]",
        "mobile:h-[4.6rem]",
        "mobile:min-w-[4.6rem]",
        "mobile:min-h-[4.6rem]",
        "mobile:text-2xl",
        "mobile:data-[special=true]:w-auto",
        "mobile:data-[special=true]:min-w-[4.6rem]",
        "mobile:data-[special=true]:text-xl",
    ],
}
