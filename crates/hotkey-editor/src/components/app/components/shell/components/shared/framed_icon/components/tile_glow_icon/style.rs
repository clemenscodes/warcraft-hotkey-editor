use tw_macro::tw;

// The glowing tile look: the shared framed base at the tile radius, lifting to a gold
// border plus soft glow when a non-disabled ancestor `.group` is hovered.
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
