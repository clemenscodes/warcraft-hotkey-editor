use tw_macro::tw;

// The glowing card look: the shared framed base at the card radius, lifting to a gold
// border plus soft glow when a non-disabled ancestor `.group` is hovered.
classes! {
    base: tw![
        "size-full",
        "overflow-hidden",
        "border",
        "border-warcraft-blue",
        "rounded-card",
        "group-[:not(:disabled):hover]:border-warcraft-gold",
        "group-[:not(:disabled):hover]:shadow-glow-soft",
    ],
}
