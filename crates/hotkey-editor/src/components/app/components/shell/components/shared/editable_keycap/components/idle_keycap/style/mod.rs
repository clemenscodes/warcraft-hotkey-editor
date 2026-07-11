use tw_macro::tw;

// The resting gold key-cap surface. It fills the box its host button hands it
// (`size-full`) and draws the whole cap — border, gold fill, glyph colour, outline
// shadow, and hover glow. Font *size* is not set here: it inherits from the host, so
// each host picks its own scale (`text-2xl` vs `text-5xl`) while the cap look stays
// shared. Focus is a pseudo of the *host* button (the focusable, keyboard-navigable
// element), so the keycap reflects it through the host's `group/editable-keycap` marker
// rather than its own `:focus-visible`. The corner radius comes from the inherited
// `--keycap-radius` (panel when the host leaves it unset).

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
        "rounded-(--keycap-radius,var(--radius-panel))",
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
    ],
}
