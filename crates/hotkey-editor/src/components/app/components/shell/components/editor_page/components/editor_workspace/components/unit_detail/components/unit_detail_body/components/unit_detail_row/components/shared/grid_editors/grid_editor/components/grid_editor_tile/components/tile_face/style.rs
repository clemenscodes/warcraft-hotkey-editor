use tw_macro::tw;
// The painter's box and the hotkey badge's query container: the square tile shape with
// rounded corners, marked a container so the badge (a descendant) sizes in `cqi` against
// this box whether or not a Host wraps the painter — the templates preview renders the
// painter alone, so the container must live here, not on the Host. All interaction visuals
// (the drag-over ring, the dragging-source ghost, the focus ring) live on the
// `GridEditorTile` Host, whose own `cqi` resolves against the outer grid; none of them
// belong to the painter.

classes! {
    base: tw![
        "relative",
        "w-full",
        "aspect-square",
        "[container-type:inline-size]",
        "rounded-[1.04cqi]",
    ],
}
