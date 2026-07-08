use tw_macro::tw;

// The empty-placeholder look: the shared framed base at the hairline radius, filled
// with the panel surface so an absent image reads as an empty framed square.
classes! {
    base: tw![
        "size-full",
        "overflow-hidden",
        "border",
        "border-warcraft-blue",
        "rounded-hairline",
        "bg-warcraft-bg-panel/70",
    ],
}
