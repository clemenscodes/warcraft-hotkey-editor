use crate::{classes, states};

use super::state::EmptyTileState;

const BASE: &[&str] = &[
    "relative",
    "w-full",
    "aspect-square",
    "[container-type:inline-size]",
    "max-w-[140px]",
    "overflow-hidden",
    "border-2",
    "rounded-[6px]",
    "touch-pan-y",
    "outline-none",
    "[body:has([data-dragging-source=true])_&]:transition-none",
];
const MOBILE: &[&str] = &["mobile:max-w-[116px]"];
const TABLET: &[&str] = &["tablet:max-w-[128px]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &["desktop:max-w-[156px]"];
const QHD: &[&str] = &["qhd:max-w-[172px]"];
const UHD: &[&str] = &["uhd:max-w-[200px]"];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }

const EMPTY: &[&str] = &[
    "[background:linear-gradient(135deg,rgba(15,30,55,0.85)_0%,rgba(8,14,30,0.85)_100%)]",
    "border-[#2c4a72]",
    "[box-shadow:inset_0_1px_0_rgba(255,255,255,0.04),0_1px_2px_rgba(0,0,0,0.5)]",
    "data-[drag-over=true]:border-warcraft-gold",
    "data-[drag-over=true]:border-dashed",
];
const DROP_TARGET: &[&str] = &[
    "[background:linear-gradient(135deg,rgba(15,30,55,0.85)_0%,rgba(8,14,30,0.85)_100%)]",
    "border-[#4a7090]",
    "border-dashed",
    "[box-shadow:inset_0_1px_0_rgba(255,255,255,0.04),0_1px_2px_rgba(0,0,0,0.5)]",
    "cursor-pointer",
    "hover:border-warcraft-gold",
    "hover:bg-[rgba(255,206,99,0.08)]",
    "data-[drag-over=true]:border-warcraft-gold",
    "data-[drag-over=true]:border-dashed",
];
const BLOCKED_DROP_TARGET: &[&str] = &[
    "[background:rgba(200,55,40,0.04)]",
    "border-[rgba(220,70,55,0.55)]",
    "border-dashed",
    "[box-shadow:inset_0_1px_0_rgba(255,255,255,0.04),0_1px_2px_rgba(0,0,0,0.5)]",
    "cursor-not-allowed",
    "data-[drag-over=true]:border-[rgba(255,95,75,0.85)]",
    "data-[drag-over=true]:[background:rgba(200,55,40,0.1)]",
];

states! {
    EmptyTileState,
    Empty => EMPTY,
    DropTarget => DROP_TARGET,
    BlockedDropTarget => BLOCKED_DROP_TARGET,
}
