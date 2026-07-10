use tw_macro::tw;
// The empty tile's own look. The resting empty slot (dark panel, solid deep-blue
// border, bevel) lives on this root. The drop-target / blocked / highlight looks are
// each a conditionally-mounted overlay child, and the ROOT reacts to which one is
// present with `:has(...)` — turning its OWN border dashed/danger/gold and its shadow
// to the glow — so the look replaces the border instead of stacking a second one, and
// the root never remounts on a state change (pointer-capture stays put through a drag).
//
// Border width and radius read `--tile-border-width` / `--tile-corner-radius` (a mini
// grid shrinks them), defaulting to the full editor tile's 2cqi / 5.2cqi. The
// drop-target look is the editor's fixed deep-blue dashed target that turns gold under
// the cursor; a parent that recolors it (the off-state picker) styles the tile through
// the `:has(.drop-target-overlay)` selector.
classes! {
    base: tw![
        "relative",
        "w-full",
        "aspect-square",
        "@container",
        "overflow-hidden",
        "border-[length:var(--tile-border-width,2cqi)]",
        "rounded-[var(--tile-corner-radius,5.2cqi)]",
        "bg-panel-dark",
        "border-warcraft-blue-deep",
        "shadow-bevel",
        "touch-pan-y",
        "outline-none",
        "[body:has(.dragging-source-ghost)_&]:transition-none",
        "[&:has(.drop-target-overlay)]:border-dashed",
        "[&:has(.drop-target-overlay)]:cursor-pointer",
        "[&:has(.drop-target-overlay):has(.drag-over-ring)]:border-warcraft-gold",
        "[&:has(.blocked-drop-target-overlay)]:border-warcraft-danger/55",
        "[&:has(.blocked-drop-target-overlay)]:border-dashed",
        "[&:has(.blocked-drop-target-overlay)]:cursor-not-allowed",
        "[&:has(.highlight-overlay)]:border-warcraft-gold",
        "[&:has(.highlight-overlay)]:[box-shadow:0_0_7cqi_color-mix(in_oklab,var(--color-warcraft-gold)_50%,transparent)]",
    ],
}
