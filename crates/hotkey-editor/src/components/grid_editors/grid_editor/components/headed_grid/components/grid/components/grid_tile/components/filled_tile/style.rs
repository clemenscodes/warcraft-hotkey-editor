use super::state::FilledTileState;
use crate::{classes, states, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "relative",
    "w-full",
    "aspect-square",
    "[container-type:inline-size]",
    "overflow-hidden",
    "border-[0.35cqi]",
    "rounded-[1.04cqi]",
    "[transition:border-color_0.12s_ease,box-shadow_0.12s_ease]",
    "touch-pan-y",
    "outline-none",
    "hover:border-warcraft-gold",
    "active:border-warcraft-gold",
    "data-[race=human]:hover:border-[color:#6aa1ff]",
    "data-[race=human]:active:border-[color:#6aa1ff]",
    "data-[race=orc]:hover:border-[color:#ff7a7a]",
    "data-[race=orc]:active:border-[color:#ff7a7a]",
    "data-[race=nightelf]:hover:border-[color:#5fdada]",
    "data-[race=nightelf]:active:border-[color:#5fdada]",
    "data-[race=undead]:hover:border-[color:#c79bff]",
    "data-[race=undead]:active:border-[color:#c79bff]",
    "data-[race=neutral]:hover:border-[color:#ffce63]",
    "data-[race=neutral]:active:border-[color:#ffce63]",
    "kb-focus:border-warcraft-gold",
    "kb-focus:[box-shadow:0_0_0_3px_#ffce63,0_0_18px_rgba(255,206,99,0.55)]",
    "data-[drag-over=true]:border-warcraft-gold",
    "data-[drag-over=true]:border-solid",
    "data-[dragging-source=true]:[background:linear-gradient(135deg,rgba(15,30,55,0.85)_0%,rgba(8,14,30,0.85)_100%)]",
    "data-[dragging-source=true]:border-[#4a7090]",
    "data-[dragging-source=true]:border-dashed",
    "data-[dragging-source=true]:[box-shadow:inset_0_1px_0_rgba(255,255,255,0.04),0_1px_2px_rgba(0,0,0,0.5)]",
    "data-[dragging-source=true]:[&>*]:invisible",
    "data-[dragging-source=true]:data-[drag-over=true]:border-warcraft-gold",
    "data-[dragging-source=true]:data-[drag-over=true]:border-dashed",
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

const FILLED: &[TailwindClass] = tw![
    "bg-[rgba(20,35,60,0.95)]",
    "border-[#4a7090]",
    "[box-shadow:inset_0_1px_0_rgba(255,255,255,0.04),0_1px_2px_rgba(0,0,0,0.5)]",
];

const COMMAND: &[TailwindClass] = tw![
    "[background:linear-gradient(135deg,rgba(40,50,80,0.95)_0%,rgba(15,22,45,0.95)_100%)]",
    "border-[#5b6f9c]",
    "[box-shadow:inset_0_1px_0_rgba(255,255,255,0.04),0_1px_2px_rgba(0,0,0,0.5)]",
];

const SELECTED: &[TailwindClass] = tw![
    "bg-[rgba(20,35,60,0.95)]",
    "border-warcraft-gold",
    "[box-shadow:0_0_14px_rgba(255,206,99,0.6)]",
    "data-[race=human]:border-[color:#6aa1ff]",
    "data-[race=human]:[box-shadow:0_0_14px_rgba(106,161,255,0.45)]",
    "data-[race=orc]:border-[color:#ff7a7a]",
    "data-[race=orc]:[box-shadow:0_0_14px_rgba(255,122,122,0.45)]",
    "data-[race=nightelf]:border-[color:#5fdada]",
    "data-[race=nightelf]:[box-shadow:0_0_14px_rgba(95,218,218,0.45)]",
    "data-[race=undead]:border-[color:#c79bff]",
    "data-[race=undead]:[box-shadow:0_0_14px_rgba(199,155,255,0.45)]",
    "data-[race=neutral]:border-[color:#ffce63]",
    "data-[race=neutral]:[box-shadow:0_0_14px_rgba(255,206,99,0.45)]",
];
states! {
    FilledTileState, Filled => FILLED, Command => COMMAND, Selected => SELECTED
}
