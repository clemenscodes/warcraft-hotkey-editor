use tw_macro::tw;

// The pulsing gold key-cap surface, lit while its key picker is open. It shares the
// resting cap's values verbatim (see `idle_keycap`) and appends the capture pulse —
// gold fill, gold border and glyph, and the strong glow. It fills the box its host
// button hands it (`size-full`); font *size* inherits from the host. Focus is a pseudo
// of the *host* button, reflected through its `group/editable-keycap` marker. The corner
// radius is the one look variant, driven by `data-radius`.

classes! {
    base: tw![
        "size-full",
        "flex",
        "items-center",
        "justify-center",
        "p-0",
        "border-2",
        "border-warcraft-gold",
        "bg-warcraft-gold-dark/75",
        "text-warcraft-gold",
        "text-shadow-outline",
        "uppercase",
        "leading-none",
        "text-center",
        "transition-[box-shadow,border-color,background,color]",
        "duration-base",
        "data-[radius=tile]:rounded-tile",
        "data-[radius=panel]:rounded-panel",
        "hover:border-warcraft-gold",
        "hover:bg-warcraft-gold/12",
        "hover:shadow-glow-soft",
        "group-focus-visible/editable-keycap:border-white",
        "group-focus-visible/editable-keycap:text-white",
        "group-focus-visible/editable-keycap:shadow-focus",
        "[@media(hover:none)]:group-focus-visible/editable-keycap:border-warcraft-gold",
        "[@media(hover:none)]:group-focus-visible/editable-keycap:bg-warcraft-gold-dark/75",
        "[@media(hover:none)]:group-focus-visible/editable-keycap:text-warcraft-gold",
        "[@media(hover:none)]:group-focus-visible/editable-keycap:shadow-none",
        "bg-panel-gold",
        "border-warcraft-gold",
        "text-warcraft-gold",
        "shadow-glow-strong",
    ],
}
