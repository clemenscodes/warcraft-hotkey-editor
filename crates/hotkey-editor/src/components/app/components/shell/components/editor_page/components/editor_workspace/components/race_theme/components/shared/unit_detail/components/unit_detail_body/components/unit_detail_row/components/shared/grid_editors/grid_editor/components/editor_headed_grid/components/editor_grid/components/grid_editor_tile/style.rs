use tw_macro::tw;
// The Host coincides with the `TileFace` painter it wraps — same square, same corners —
// so the focus ring sits exactly over the drawn tile. The dragging-source, drag-over,
// and drop-target looks are ALL the painter's own border now (driven by the overlay
// children the Host's drag flags mount inside the painter), so they replace the tile
// border instead of stacking a ring, exactly as production does. The query container
// itself lives on the painter (so its badge sizes with no Host, in the preview); the
// Host has none, so its own `cqi` overlays resolve against the outer grid.
//
// The cursor tracks draggability via the mounted `DraggableMarker`: `grab` only on a
// draggable tile (`:has(.draggable-marker)`), the default arrow otherwise — an empty
// tile is not draggable, so it reads as inert — and `grabbing` while any drag is in
// flight (`body:has(.dragging-source-ghost)`).

classes! {
    base: tw![
        "relative",
        "w-full",
        "aspect-square",
        "rounded-[1.27cqi]",
        "touch-pan-y",
        "cursor-default",
        "[&:has(.draggable-marker)]:cursor-grab",
        "outline-none",
        "kb-focus:[box-shadow:0_0_0_0.52cqi_var(--color-warcraft-gold),0_0_3.1cqi_color-mix(in_oklab,var(--color-warcraft-gold)_55%,transparent)]",
        "[body:has(.dragging-source-ghost)_&]:cursor-grabbing",
    ],
}
