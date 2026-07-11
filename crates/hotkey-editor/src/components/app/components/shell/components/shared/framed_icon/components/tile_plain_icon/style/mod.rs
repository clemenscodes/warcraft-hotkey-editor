use tw_macro::tw;

// The plain tile look: the shared framed base (fills its box, clips its image, blue
// border) at the tile radius, with no hover glow.
classes! {
    base: tw![
        "size-full",
        "overflow-hidden",
        "border",
        "border-warcraft-blue",
        "rounded-tile",
    ],
}
