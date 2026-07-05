use super::state::EmptyTileState;
use crate::{classes, states, styling::TailwindClass, tw};

// Sizes in `cqi` off the grid's container: the tile fills its column and its
// border and corner scale with the grid, so the same tile renders full-size in the
// editor and tiny in a mini grid.
const BASE: &[TailwindClass] = tw![
    "relative",
    "w-full",
    "aspect-square",
    "[container-type:inline-size]",
    "overflow-hidden",
    "border-[0.35cqi]",
    "rounded-[1.04cqi]",
    "touch-pan-y",
    "outline-none",
    "[body:has([data-dragging-source=true])_&]:transition-none",
];

const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}

const EMPTY: &[TailwindClass] = tw![
    "bg-panel-dark-diag-85",
    "border-warcraft-blue-bright-deep",
    "shadow-bevel-hl",
];

const DROP_TARGET: &[TailwindClass] = tw![
    "bg-panel-dark-diag-85",
    "border-warcraft-blue-bright",
    "border-dashed",
    "shadow-bevel-hl",
    "cursor-pointer",
    "hover:border-warcraft-gold",
    "hover:bg-warcraft-gold/8",
];

const BLOCKED_DROP_TARGET: &[TailwindClass] = tw![
    "[background:color-mix(in_oklab,var(--color-warcraft-danger)_4%,transparent)]",
    "border-warcraft-danger/55",
    "border-dashed",
    "shadow-bevel-hl",
    "cursor-not-allowed",
];

// The mini grid marks one coordinate: a gold-accented border, gold wash, and glow,
// all scaling with the grid via `cqi`. Mini grids sit outside any race context, so
// the accent is the fixed warcraft gold (there is no race to tint it).
const HIGHLIGHTED: &[TailwindClass] = tw![
    "border-warcraft-gold",
    "bg-warcraft-gold/20",
    "[box-shadow:0_0_7cqi_color-mix(in_oklab,var(--color-warcraft-gold)_50%,transparent)]",
];

states! {
    EmptyTileState, Empty => EMPTY, DropTarget => DROP_TARGET, BlockedDropTarget =>
    BLOCKED_DROP_TARGET, Highlighted => HIGHLIGHTED,
}
