use tw_macro::tw;
// The Host coincides with the `TileFace` painter it wraps — same square, same corners —
// so the dragging-source ghost and focus ring sit exactly over the drawn tile. The
// drop-target and drag-over looks are the painter's own border (driven off the Host's
// `data-drag-over` / `data-dragging-source` markers), so they replace the tile border
// instead of stacking a ring over it, exactly as production does. The query container
// itself lives on the painter (so its badge sizes with no Host, in the preview); the
// Host has none, so its own `cqi` overlays resolve against the outer grid.
//
// The cursor tracks draggability: `grab` only on a draggable tile (`data-draggable=true`),
// the default arrow otherwise — an empty tile is not draggable, so it reads as inert — and
// `grabbing` while any drag is in flight.

classes! {
    base: tw![
        "relative",
        "w-full",
        "aspect-square",
        "rounded-[1.27cqi]",
        "touch-pan-y",
        "cursor-default",
        "data-[draggable=true]:cursor-grab",
        "outline-none",
        "kb-focus:[box-shadow:0_0_0_0.52cqi_var(--color-warcraft-gold),0_0_3.1cqi_color-mix(in_oklab,var(--color-warcraft-gold)_55%,transparent)]",
        "[body:has([data-dragging-source=true])_&]:cursor-grabbing",
        "[data-grid-row]",
        "data-[dragging-source=true]:[&>*]:opacity-0",
        "data-[dragging-source=true]:border-[0.47cqi]",
        "data-[dragging-source=true]:border-dashed",
        "data-[dragging-source=true]:border-warcraft-blue-slate",
        "data-[dragging-source=true]:bg-panel-dark-diag-85",
        "data-[dragging-source=true]:shadow-bevel-hl",
        "data-[dragging-source=true]:data-[drag-over=true]:border-warcraft-gold",
    ],
}
