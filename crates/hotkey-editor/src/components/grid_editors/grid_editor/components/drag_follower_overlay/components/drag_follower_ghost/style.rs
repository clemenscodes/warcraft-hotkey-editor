use super::state::GhostState;
use crate::{classes, states};

const BASE: &[&str] = &[
    "fixed",
    "pointer-events-none",
    "[container-type:inline-size]",
    "z-[1000]",
    "overflow-hidden",
    "select-none",
    "border-2",
    "rounded-[6px]",
    "border-[color:var(--race-color,#ffce63)]",
    "[box-shadow:0_0_14px_var(--race-color-soft,rgba(255,206,99,0.6)),0_8px_24px_rgba(0,0,0,0.6)]",
    "data-[race=human]:[--race-color:#6aa1ff]",
    "data-[race=human]:[--race-color-soft:rgba(106,161,255,0.45)]",
    "data-[race=orc]:[--race-color:#ff7a7a]",
    "data-[race=orc]:[--race-color-soft:rgba(255,122,122,0.45)]",
    "data-[race=nightelf]:[--race-color:#5fdada]",
    "data-[race=nightelf]:[--race-color-soft:rgba(95,218,218,0.45)]",
    "data-[race=undead]:[--race-color:#c79bff]",
    "data-[race=undead]:[--race-color-soft:rgba(199,155,255,0.45)]",
    "data-[race=neutral]:[--race-color:#ffce63]",
    "data-[race=neutral]:[--race-color-soft:rgba(255,206,99,0.45)]",
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

const DEFAULT: &[&str] = &["bg-[rgba(20,35,60,0.95)]"];

const COMMAND: &[&str] =
    &["[background:linear-gradient(135deg,rgba(40,50,80,0.95)_0%,rgba(15,22,45,0.95)_100%)]"];
states! {
    GhostState, Default => DEFAULT, Command => COMMAND
}
