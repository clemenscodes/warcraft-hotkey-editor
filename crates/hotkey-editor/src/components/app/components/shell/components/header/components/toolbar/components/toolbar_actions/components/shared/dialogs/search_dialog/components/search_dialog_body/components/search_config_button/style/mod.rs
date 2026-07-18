use tw_macro::tw;

// A quiet disclosure trigger that sits under the search field: as wide as its own
// label, never stretched across the dialog, so it reads as a control rather than a
// bar. The colour and focus treatment mirror the scope trigger's, because both are
// small text buttons over the dialog surface.
classes! {
    base: tw![
        "flex",
        "items-center",
        "gap-1",
        "self-start",
        "px-2",
        "py-1",
        "bg-transparent",
        "border-none",
        "text-warcraft-gold",
        "text-sm",
        "uppercase",
        "tracking-caps",
        "cursor-pointer",
        "transition-[color]",
        "hover:text-white",
        "kb-focus:outline-none",
        "kb-focus:text-white",
        "kb-focus:[--focus-color:var(--color-warcraft-highlight)]",
        "kb-focus:shadow-focus",
    ],
}
