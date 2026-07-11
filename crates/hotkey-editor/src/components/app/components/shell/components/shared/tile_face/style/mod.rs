use tw_macro::tw;
// The painter's box and the hotkey badge's query container: the square tile shape with
// rounded corners, marked a container so the badge (a descendant) sizes in `cqi` against
// this box whether or not a Host wraps the painter — the templates preview renders the
// painter alone, so the container must live here, not on the Host. The drag-over and
// dragging-source looks are the base tile's own border (driven by the mounted overlay
// children inside `GridTile`); the one thing this wrapper adds is hiding the hotkey
// badge while the tile is the lifted drag source, since the opaque ghost covers the
// icon but the badge is this wrapper's own child, drawn above it. The focus ring lives
// on the `GridEditorTile` Host.

classes! {
    base: tw![
        "relative",
        "w-full",
        "aspect-square",
        "@container",
        "rounded-[1.04cqi]",
        "[&:has(.dragging-source-ghost)_.tile-badge]:invisible",
    ],
}
