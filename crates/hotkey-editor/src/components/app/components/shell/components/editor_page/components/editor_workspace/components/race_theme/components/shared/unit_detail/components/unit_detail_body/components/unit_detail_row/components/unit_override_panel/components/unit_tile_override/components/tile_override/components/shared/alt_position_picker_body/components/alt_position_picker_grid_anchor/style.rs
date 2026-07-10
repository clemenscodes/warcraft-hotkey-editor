use tw_macro::tw;
// Centers the embedded command grid inside a position-picker dialog and picker-styles
// its tiles for the single-button off-state drag: non-draggable tiles dim out, and the
// drop candidates glow gold. It keys off the tiles' mounted markers — the Host's
// `.draggable-marker`, and the painter's `.drop-target-overlay` / `.drag-over-ring` /
// `.dragging-source-ghost` (the overlay children that replaced the old
// `data-draggable` / `data-drag-over` / `data-dragging-source` / `data-drop-target`
// look-flags). These beat the tile's own single-`:has` rules by descendant specificity,
// so no `!important` is needed.
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
        "[&_.grid-editor-tile:not(:has(.draggable-marker))_.filled-tile]:cursor-default",
        "[&_.grid-editor-tile:not(:has(.draggable-marker))_.filled-tile]:opacity-[0.32]",
        "[&_.grid-editor-tile:not(:has(.draggable-marker))_.filled-tile]:filter-[saturate(0.35)_brightness(0.85)]",
        "[&_.grid-editor-tile:not(:has(.draggable-marker))_.filled-tile]:border-warcraft-blue-deep",
        "[&_.grid-editor-tile:not(:has(.draggable-marker))_.filled-tile]:shadow-bevel",
        "[&_.grid-editor-tile:not(:has(.draggable-marker))_.filled-tile]:transform-none",
        "[&_.grid-editor-tile:not(:has(.draggable-marker))_.empty-tile:not(:has(.drop-target-overlay))]:[border:2px_dashed_color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent)]",
        "[&_.grid-editor-tile:not(:has(.draggable-marker))_.empty-tile:not(:has(.drop-target-overlay))]:[background:color-mix(in_oklab,var(--color-warcraft-gold)_4%,transparent)]",
        "[&_.grid-editor-tile:not(:has(.draggable-marker)):hover_.empty-tile:not(:has(.drop-target-overlay))]:border-warcraft-gold/75",
        "[&_.grid-editor-tile:not(:has(.draggable-marker)):hover_.empty-tile:not(:has(.drop-target-overlay))]:[background:color-mix(in_oklab,var(--color-warcraft-gold)_10%,transparent)]",
        "[&_.empty-tile:has(.drop-target-overlay)]:[border:2px_dashed_color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent)]",
        "[&_.empty-tile:has(.drop-target-overlay)]:[background:color-mix(in_oklab,var(--color-warcraft-gold)_4%,transparent)]",
        "[&_.empty-tile:has(.drop-target-overlay)]:cursor-pointer",
        "[&_.empty-tile:has(.drop-target-overlay)]:shadow-glow-soft",
        "[&_.empty-tile:has(.drop-target-overlay):has(.drag-over-ring)]:[border:2px_solid_var(--color-warcraft-gold)]",
        "[&_.empty-tile:has(.drop-target-overlay):has(.drag-over-ring)]:bg-panel-gold",
        "[&_.empty-tile:has(.drop-target-overlay):has(.drag-over-ring)]:shadow-ring",
        "[&_.grid-editor-tile:has(.draggable-marker)_.filled-tile]:cursor-grab",
        "[&_.grid-editor-tile:has(.draggable-marker)_.filled-tile]:border-warcraft-gold",
        "[&_.grid-editor-tile:has(.draggable-marker)_.filled-tile]:transition-none",
        "[&_.grid-editor-tile:has(.draggable-marker):active_.filled-tile]:cursor-grabbing",
        "[&_.filled-tile:has(.dragging-source-ghost)]:[border:2px_dashed_color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent)]",
        "[&_.filled-tile:has(.dragging-source-ghost)]:[background:color-mix(in_oklab,var(--color-warcraft-gold)_4%,transparent)]",
        "[&_.filled-tile:has(.dragging-source-ghost)]:shadow-none",
        "[&_.filled-tile:has(.dragging-source-ghost):has(.drag-over-ring)]:[border:2px_solid_var(--color-warcraft-gold)]",
        "[&_.filled-tile:has(.dragging-source-ghost):has(.drag-over-ring)]:bg-panel-gold",
        "[&_.filled-tile:has(.dragging-source-ghost):has(.drag-over-ring)]:shadow-ring",
    ],
}
