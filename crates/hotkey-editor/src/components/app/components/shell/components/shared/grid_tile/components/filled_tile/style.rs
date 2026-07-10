use tw_macro::tw;
// The occupied tile's own look. The resting filled border (deep blue) and bevel live
// on this root; the ability-vs-command BACKGROUND is a child fill (`AbilityFill` /
// `CommandFill`). Every other state is a mounted overlay child the root reacts to with
// `:has(...)`, never a class swap, so the root never remounts (pointer capture stays put
// through a drag):
//   - SELECTION (`SelectionRing`) turns the border gold and glows.
//   - the DRAGGING SOURCE (`DraggingSourceGhost`, an opaque cover) turns the border into
//     the dashed deep-blue ghost, matching an empty drop target.
//   - the drag cursor being over this tile (`DragOverRing`) turns the border gold — solid
//     on a swap target, dashed when the hovered tile is the source itself.
// Border width and corner radius read `--tile-border-width` / `--tile-corner-radius` (a
// parent may shrink them for a mini grid), defaulting to the full editor tile's 2cqi /
// 5.2cqi. `isolate` scopes the fills' `-z-10` behind the glyph.
classes! {
    base: tw![
        "relative",
        "isolate",
        "w-full",
        "aspect-square",
        "@container",
        "overflow-hidden",
        "border-(length:--tile-border-width,2cqi)",
        "rounded-(--tile-corner-radius,5.2cqi)",
        "border-warcraft-blue",
        "shadow-bevel",
        "transition-[border-color,box-shadow]",
        "duration-fast",
        "touch-pan-y",
        "outline-none",
        "hover:border-(--race-accent,var(--color-warcraft-gold))",
        "active:border-(--race-accent,var(--color-warcraft-gold))",
        "kb-focus:border-warcraft-gold",
        "kb-focus:shadow-focus",
        "has-[.selection-ring]:border-(--race-accent,var(--color-warcraft-gold))",
        "has-[.selection-ring]:shadow-glow",
        "has-[.selection-ring]:[--glow-color:var(--race-accent,var(--color-warcraft-gold))]",
        "has-[.dragging-source-ghost]:border-warcraft-blue-deep",
        "has-[.dragging-source-ghost]:border-dashed",
        "has-[.dragging-source-ghost]:bg-panel-dark",
        "[&:has(.dragging-source-ghost)>*]:invisible",
        "has-[.drag-over-ring]:border-warcraft-gold",
        "has-[.drag-over-ring]:border-solid",
        "[&:has(.dragging-source-ghost):has(.drag-over-ring)]:border-warcraft-gold",
        "[&:has(.dragging-source-ghost):has(.drag-over-ring)]:border-dashed",
        "[body:has(.dragging-source-ghost)_&]:transition-none",
    ],
}
