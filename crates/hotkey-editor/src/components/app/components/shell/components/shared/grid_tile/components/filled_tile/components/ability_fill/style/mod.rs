use tw_macro::tw;
// The background layer of an ordinary ability (and of a selected) tile. An inset,
// clipped fill that sits behind the icon and label (`-z-10`, against the tile root's
// `isolate` context) so a command tile's opaque fill never paints over its glyph.
classes! {
    base: tw![
        "absolute",
        "inset-0",
        "-z-10",
        "bg-warcraft-bg-panel/95",
    ],
}
