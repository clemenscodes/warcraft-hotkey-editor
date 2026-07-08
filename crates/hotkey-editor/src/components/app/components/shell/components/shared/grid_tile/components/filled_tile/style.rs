use tw_macro::tw;
// The occupied tile's own look. The resting filled border (deep blue) and bevel live
// on this root; the ability-vs-command BACKGROUND is a child fill (`AbilityFill` /
// `CommandFill`), and SELECTION is the mounted `SelectionRing` child — the root turns
// its own border gold and glows via `:has(.selection-ring)`, so the selected look
// replaces the border instead of stacking a ring, and the root never remounts on a
// state change. Border width and corner radius read `--tile-border-width` /
// `--tile-corner-radius` (a parent may shrink them for a mini grid), defaulting to the
// full editor tile's 2cqi / 5.2cqi. `isolate` scopes the fills' `-z-10` behind the
// glyph.
classes! {
    base: tw![
        "relative",
        "isolate",
        "w-full",
        "aspect-square",
        "@container",
        "overflow-hidden",
        "border-[length:var(--tile-border-width,2cqi)]",
        "rounded-[var(--tile-corner-radius,5.2cqi)]",
        "border-warcraft-blue",
        "shadow-bevel",
        "transition-[border-color,box-shadow]", "duration-fast",
        "touch-pan-y",
        "outline-none",
        "hover:border-warcraft-gold",
        "active:border-warcraft-gold",
        "data-[race=human]:hover:border-race-human",
        "data-[race=human]:active:border-race-human",
        "data-[race=orc]:hover:border-race-orc",
        "data-[race=orc]:active:border-race-orc",
        "data-[race=nightelf]:hover:border-race-nightelf",
        "data-[race=nightelf]:active:border-race-nightelf",
        "data-[race=undead]:hover:border-race-undead",
        "data-[race=undead]:active:border-race-undead",
        "data-[race=neutral]:hover:border-warcraft-gold",
        "data-[race=neutral]:active:border-warcraft-gold",
        "kb-focus:border-warcraft-gold",
        "kb-focus:shadow-focus",
        "[&:has(.selection-ring)]:border-warcraft-gold",
        "[&:has(.selection-ring)]:shadow-glow",
        "[&:has(.selection-ring)]:data-[race=human]:border-race-human",
        "[&:has(.selection-ring)]:data-[race=human]:[--glow-color:var(--color-race-human)]",
        "[&:has(.selection-ring)]:data-[race=orc]:border-race-orc",
        "[&:has(.selection-ring)]:data-[race=orc]:[--glow-color:var(--color-race-orc)]",
        "[&:has(.selection-ring)]:data-[race=nightelf]:border-race-nightelf",
        "[&:has(.selection-ring)]:data-[race=nightelf]:[--glow-color:var(--color-race-nightelf)]",
        "[&:has(.selection-ring)]:data-[race=undead]:border-race-undead",
        "[&:has(.selection-ring)]:data-[race=undead]:[--glow-color:var(--color-race-undead)]",
        "[&:has(.selection-ring)]:data-[race=neutral]:border-warcraft-gold",
        "in-data-[drag-over=true]:border-warcraft-gold",
        "in-data-[drag-over=true]:border-solid",
        "data-[dragging-source=true]:bg-panel-dark",
        "data-[dragging-source=true]:border-warcraft-blue",
        "data-[dragging-source=true]:border-dashed",
        "data-[dragging-source=true]:shadow-bevel",
        "data-[dragging-source=true]:*:invisible",
        "data-[dragging-source=true]:data-[drag-over=true]:border-warcraft-gold",
        "data-[dragging-source=true]:data-[drag-over=true]:border-dashed",
        "[body:has([data-dragging-source=true])_&]:transition-none",
    ],
}
