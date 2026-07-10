use tw_macro::tw;
// Centers the embedded command grid inside a position-picker dialog and picker-styles
// its tiles for the single-button off-state drag: non-draggable tiles dim out, and the
// drop candidates glow gold. It keys off the tiles' KEPT interaction seams — the Host's
// `data-draggable` / `data-drag-over` / `data-dragging-source` markers — and off the
// empty tile's mounted `.drop-target-overlay` (which replaced the old
// `data-drop-target` look-flag). These beat the tile's own single-`:has` rules by
// descendant specificity, so no `!important` is needed.
classes! {
    base: tw![
        "flex",
        "justify-center",
        "w-full",
        "[&_.grid-section]:@container-normal",
        "[&_.grid-section]:w-max",
        "[&_.grid]:[--tile-size:8rem]",
        "[&_.grid]:grid-cols-[repeat(4,var(--tile-size))]",
        "[&_.grid]:auto-rows-(--tile-size)",
        "[&_.filled-tile]:w-(--tile-size)",
        "[&_.filled-tile]:h-(--tile-size)",
        "[&_.empty-tile]:w-(--tile-size)",
        "[&_.empty-tile]:h-(--tile-size)",
        "[&_.command-tile-wrapper]:w-(--tile-size)",
        "[&_.command-tile-wrapper]:h-(--tile-size)",
        "[&_.grid-editor-tile[data-draggable=false]_.filled-tile]:cursor-default",
        "[&_.grid-editor-tile[data-draggable=false]_.filled-tile]:opacity-[0.32]",
        "[&_.grid-editor-tile[data-draggable=false]_.filled-tile]:filter-[saturate(0.35)_brightness(0.85)]",
        "[&_.grid-editor-tile[data-draggable=false]_.filled-tile]:border-warcraft-blue-deep",
        "[&_.grid-editor-tile[data-draggable=false]_.filled-tile]:shadow-bevel",
        "[&_.grid-editor-tile[data-draggable=false]_.filled-tile]:transform-none",
        "[&_.grid-editor-tile[data-draggable=false]_.empty-tile:not(:has(.drop-target-overlay))]:[border:2px_dashed_color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent)]",
        "[&_.grid-editor-tile[data-draggable=false]_.empty-tile:not(:has(.drop-target-overlay))]:[background:color-mix(in_oklab,var(--color-warcraft-gold)_4%,transparent)]",
        "[&_.grid-editor-tile[data-draggable=false]:hover_.empty-tile:not(:has(.drop-target-overlay))]:border-warcraft-gold/75",
        "[&_.grid-editor-tile[data-draggable=false]:hover_.empty-tile:not(:has(.drop-target-overlay))]:[background:color-mix(in_oklab,var(--color-warcraft-gold)_10%,transparent)]",
        "[&_.empty-tile:has(.drop-target-overlay)]:[border:2px_dashed_color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent)]",
        "[&_.empty-tile:has(.drop-target-overlay)]:[background:color-mix(in_oklab,var(--color-warcraft-gold)_4%,transparent)]",
        "[&_.empty-tile:has(.drop-target-overlay)]:cursor-pointer",
        "[&_.empty-tile:has(.drop-target-overlay)]:shadow-glow-soft",
        "[&_.grid-editor-tile[data-drag-over=true]_.empty-tile]:[border:2px_solid_var(--color-warcraft-gold)]",
        "[&_.grid-editor-tile[data-drag-over=true]_.empty-tile]:bg-panel-gold",
        "[&_.grid-editor-tile[data-drag-over=true]_.empty-tile]:shadow-ring",
        "[&_.grid-editor-tile[data-draggable=true]_.filled-tile]:cursor-grab",
        "[&_.grid-editor-tile[data-draggable=true]_.filled-tile]:border-warcraft-gold",
        "[&_.grid-editor-tile[data-draggable=true]_.filled-tile]:transition-none",
        "[&_.grid-editor-tile[data-draggable=true]:active_.filled-tile]:cursor-grabbing",
        "[&_.grid-editor-tile[data-draggable=true][data-dragging-source=true]]:[border:2px_dashed_color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent)]",
        "[&_.grid-editor-tile[data-draggable=true][data-dragging-source=true]]:[background:color-mix(in_oklab,var(--color-warcraft-gold)_4%,transparent)]",
        "[&_.grid-editor-tile[data-draggable=true][data-dragging-source=true]]:shadow-none",
        "[&_.grid-editor-tile[data-draggable=true][data-dragging-source=true][data-drag-over=true]]:[border:2px_solid_var(--color-warcraft-gold)]",
        "[&_.grid-editor-tile[data-draggable=true][data-dragging-source=true][data-drag-over=true]]:bg-panel-gold",
        "[&_.grid-editor-tile[data-draggable=true][data-dragging-source=true][data-drag-over=true]]:shadow-ring",
    ],
}
