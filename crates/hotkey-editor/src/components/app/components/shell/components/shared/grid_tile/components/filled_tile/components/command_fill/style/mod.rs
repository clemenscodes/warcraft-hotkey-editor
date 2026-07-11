use tw_macro::tw;
// The background layer of a built-in command tile (build, cancel, …). An inset,
// clipped fill behind the icon and label (`-z-10`, against the tile root's `isolate`
// context), so its opaque blue panel never paints over the command glyph.
classes! {
    base: tw![
        "absolute",
        "inset-0",
        "-z-10",
        "bg-panel-blue",
    ],
}
