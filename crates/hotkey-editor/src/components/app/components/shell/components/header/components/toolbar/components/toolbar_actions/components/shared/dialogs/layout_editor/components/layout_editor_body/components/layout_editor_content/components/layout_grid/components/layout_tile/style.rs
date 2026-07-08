use tw_macro::tw;
// The layout tile's host button: the focusable, draggable, keyboard-navigable box
// around the shared `EditableKeycap`. It owns only the *box* — it fills the grid cell
// it is given (`size-full`), sets the font size the cap inherits, and suppresses the
// focus outline. The gold cap look and the capture pulse live on the nested
// `EditableKeycap`; the `group/editable-keycap` marker lets that cap reflect this
// button's keyboard focus. Class `.layout-tile` is load-bearing (keyboard navigation).

classes! {
    base: tw![
        "group/editable-keycap",
        "flex",
        "items-center",
        "justify-center",
        "size-full",
        "p-0",
        "text-5xl",
        "focus:outline-none",
        "kb-focus:outline-none",
    ],
    mobile: tw![
        "mobile:text-2xl",
    ],
    tablet: tw![
        "tablet:text-2xl",
    ],
}
