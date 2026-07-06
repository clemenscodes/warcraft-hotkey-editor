use super::state::EmptyTileState;
use tw_macro::tw;

// Border and corner in `cqi` off the tile-face container (one tile), so they read
// as production's 2px border / 6px radius on a full-size editor tile. Mini grids
// render this painter with NO tile-face, so their frame overrides both back down
// (see each `mini_grid/style.rs`).

classes! {
    base: tw![
        "relative",
        "w-full",
        "aspect-square",
        "[container-type:inline-size]",
        "overflow-hidden",
        "border-[2cqi]",
        "rounded-[5.2cqi]",
        "touch-pan-y",
        "outline-none",
        "[body:has([data-dragging-source=true])_&]:transition-none",
    ],
}

// A drop target during a drag looks exactly like the lifted source tile: a
// muted-slate dashed border of the same weight, no differentiation. Under the
// cursor (`data-drag-over` on the wrapping Host) the same border turns gold —
// replacing the slate, not stacking a ring over it, matching production.

// The mini grid marks one coordinate: a gold-accented border, gold wash, and glow,
// all scaling with the grid via `cqi`. Mini grids sit outside any race context, so
// the accent is the fixed warcraft gold (there is no race to tint it).

states! {
    EmptyTileState,
    Empty => tw![
        "bg-panel-dark-diag-85",
        "border-warcraft-blue-bright-deep",
        "shadow-bevel-hl",
    ],
    DropTarget => tw![
        "bg-panel-dark-diag-85",
        "border-warcraft-blue-slate",
        "border-dashed",
        "shadow-bevel-hl",
        "cursor-pointer",
        "[[data-drag-over=true]_&]:border-warcraft-gold",
    ],
    BlockedDropTarget => tw![
        "[background:color-mix(in_oklab,var(--color-warcraft-danger)_4%,transparent)]",
        "border-warcraft-danger/55",
        "border-dashed",
        "shadow-bevel-hl",
        "cursor-not-allowed",
    ],
    Highlighted => tw![
        "border-warcraft-gold",
        "bg-warcraft-gold/20",
        "[box-shadow:0_0_7cqi_color-mix(in_oklab,var(--color-warcraft-gold)_50%,transparent)]",
    ],
}
