use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "flex",
        "flex-row",
        "min-w-0",
        "h-14",
        "rounded-panel",
        "border",
        "border-warcraft-gold-border",
        "overflow-hidden",
        "[&>*+*]:border-l",
        "[&>*+*]:border-warcraft-gold-border",
    ],
}
