use super::state::GhostState;
use crate::{classes, states, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "fixed",
    "pointer-events-none",
    "[container-type:inline-size]",
    "z-[1000]",
    "overflow-hidden",
    "select-none",
    "border-2",
    "rounded-[6px]",
    "border-warcraft-gold",
    "[box-shadow:0_0_14px_color-mix(in_oklab,var(--color-warcraft-gold)_60%,transparent),0_8px_24px_color-mix(in_oklab,var(--color-warcraft-shadow)_60%,transparent)]",
    "data-[race=human]:border-[color:var(--color-race-human)]",
    "data-[race=human]:[box-shadow:0_0_14px_color-mix(in_oklab,var(--color-race-human)_45%,transparent),0_8px_24px_color-mix(in_oklab,var(--color-warcraft-shadow)_60%,transparent)]",
    "data-[race=orc]:border-[color:var(--color-race-orc)]",
    "data-[race=orc]:[box-shadow:0_0_14px_color-mix(in_oklab,var(--color-race-orc)_45%,transparent),0_8px_24px_color-mix(in_oklab,var(--color-warcraft-shadow)_60%,transparent)]",
    "data-[race=nightelf]:border-[color:var(--color-race-nightelf)]",
    "data-[race=nightelf]:[box-shadow:0_0_14px_color-mix(in_oklab,var(--color-race-nightelf)_45%,transparent),0_8px_24px_color-mix(in_oklab,var(--color-warcraft-shadow)_60%,transparent)]",
    "data-[race=undead]:border-[color:var(--color-race-undead)]",
    "data-[race=undead]:[box-shadow:0_0_14px_color-mix(in_oklab,var(--color-race-undead)_45%,transparent),0_8px_24px_color-mix(in_oklab,var(--color-warcraft-shadow)_60%,transparent)]",
    "data-[race=neutral]:border-[color:var(--color-warcraft-gold)]",
    "data-[race=neutral]:[box-shadow:0_0_14px_color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent),0_8px_24px_color-mix(in_oklab,var(--color-warcraft-shadow)_60%,transparent)]",
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

const DEFAULT: &[TailwindClass] = tw!["bg-warcraft-bg-panel/95"];

const COMMAND: &[TailwindClass] = tw![
    "bg-panel-blue-diag-95"
];
states! {
    GhostState, Default => DEFAULT, Command => COMMAND
}
