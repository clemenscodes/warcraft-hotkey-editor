use super::state::HotkeyBadgeState;
use crate::{classes, states, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "inline-flex",
    "items-center",
    "justify-center",
    "min-w-[24cqi]",
    "h-[24cqi]",
    "px-[5cqi]",
    "rounded-[5cqi]",
    "border",
    "text-[17cqi]/[1]",
    "font-normal",
    "pointer-events-none",
    "[text-shadow:1px_1px_0_var(--color-warcraft-shadow)]",
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

const NORMAL: &[TailwindClass] = tw![
    "bg-warcraft-shadow/78",
    "border-warcraft-gold/55",
    "text-warcraft-gold",
];

const PASSIVE: &[TailwindClass] = tw![
    "bg-warcraft-bg-mid",
    "border-warcraft-blue-glow",
    "text-warcraft-text-secondary"
];

const CONFLICT: &[TailwindClass] = tw![
    "bg-race-orc-strong/85",
    "border-warcraft-danger",
    "text-warcraft-danger",
];
states! {
    HotkeyBadgeState, Normal => NORMAL, Passive => PASSIVE, Conflict => CONFLICT
}
