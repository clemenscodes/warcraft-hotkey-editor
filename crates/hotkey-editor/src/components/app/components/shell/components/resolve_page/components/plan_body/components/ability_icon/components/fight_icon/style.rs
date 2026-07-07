use tw_macro::tw;
classes! {
    base: tw![
        "size-full",
        "border",
        "border-warcraft-blue",
        "rounded-card",
        "object-cover",
        "group-[:not(:disabled):hover]:border-warcraft-gold",
        "group-[:not(:disabled):hover]:shadow-glow-soft",
    ],
}
