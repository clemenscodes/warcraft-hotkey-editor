use tw_macro::tw;

classes! {
    base: tw![
        "size-full",
        "overflow-hidden",
        "border",
        "border-warcraft-blue",
        "rounded-tile",
        "group-[:not(:disabled):hover]:border-warcraft-gold",
        "group-[:not(:disabled):hover]:shadow-glow-soft",
    ],
}
