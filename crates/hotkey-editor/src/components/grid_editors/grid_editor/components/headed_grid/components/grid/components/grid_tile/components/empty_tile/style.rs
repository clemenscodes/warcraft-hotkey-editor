use super::state::EmptyTileState;
use crate::{classes, states};

// Sizes in `cqi` off the grid's container: the tile fills its column and its
// border and corner scale with the grid, so the same tile renders full-size in the
// editor and tiny in a mini grid.
const BASE: &[&str] = &[
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

const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}

const EMPTY: &[&str] = &[
    "[background:linear-gradient(135deg,rgba(15,30,55,0.85)_0%,rgba(8,14,30,0.85)_100%)]",
    "border-[#2c4a72]",
    "[box-shadow:inset_0_1px_0_rgba(255,255,255,0.04),0_1px_2px_rgba(0,0,0,0.5)]",
];

const DROP_TARGET: &[&str] = &[
    "[background:linear-gradient(135deg,rgba(15,30,55,0.85)_0%,rgba(8,14,30,0.85)_100%)]",
    "border-[#4a7090]",
    "border-dashed",
    "[box-shadow:inset_0_1px_0_rgba(255,255,255,0.04),0_1px_2px_rgba(0,0,0,0.5)]",
    "cursor-pointer",
    "hover:border-warcraft-gold",
    "hover:bg-[rgba(255,206,99,0.08)]",
];

const BLOCKED_DROP_TARGET: &[&str] = &[
    "[background:rgba(200,55,40,0.04)]",
    "border-[rgba(220,70,55,0.55)]",
    "border-dashed",
    "[box-shadow:inset_0_1px_0_rgba(255,255,255,0.04),0_1px_2px_rgba(0,0,0,0.5)]",
    "cursor-not-allowed",
];

// The mini grid marks one coordinate: the race-accented border, gold wash, and
// glow, all scaling with the grid via `cqi`.
const HIGHLIGHTED: &[&str] = &[
    "[border-color:var(--race-color,#ffce63)]",
    "bg-[rgba(255,206,99,0.2)]",
    "[box-shadow:0_0_7cqi_var(--race-color-soft,rgba(255,206,99,0.5))]",
];

states! {
    EmptyTileState, Empty => EMPTY, DropTarget => DROP_TARGET, BlockedDropTarget =>
    BLOCKED_DROP_TARGET, Highlighted => HIGHLIGHTED,
}
