use tw_macro::tw;
// The multi-character override key's host button: the widened, focusable box around the
// shared `EditableKeycap`, sized to fit special tokens (Esc, Mouse4) whose labels exceed
// the square letter box. It owns only the box — the widened size, the font size the cap
// inherits, focus suppression, and the soft focus glow. The gold cap look and the
// capture pulse live on the nested `EditableKeycap`; the `group/editable-keycap` marker
// lets that cap reflect this button's keyboard focus. Class `.special-override-key` is
// load-bearing (keyboard navigation).
classes! {
    base: tw![
        "group/editable-keycap",
        "[--keycap-radius:var(--radius-tile)]",
        "flex",
        "items-center",
        "justify-center",
        "w-auto",
        "min-w-20",
        "h-20",
        "p-0",
        "text-xl",
        "whitespace-nowrap",
        "tracking-normal",
        "cursor-pointer",
        "kb-focus:outline-none",
        "kb-focus:shadow-glow-soft",
    ],
    mobile: tw![
        "mobile:w-auto",
        "mobile:h-[4.6rem]",
        "mobile:min-w-[4.6rem]",
        "mobile:min-h-[4.6rem]",
        "mobile:text-xl",
    ],
}
