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
    "font-friz-quadrata",
    "text-[17cqi]/[1]",
    "font-normal",
    "pointer-events-none",
    "[text-shadow:1px_1px_0_#000]",
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
    "bg-[rgba(0,0,0,0.78)]",
    "border-warcraft-gold/55",
    "text-warcraft-gold",
];

const PASSIVE: &[TailwindClass] = tw!["bg-[#1a1f29]", "border-[#4a5160]", "text-[#b8bcc4]"];

const CONFLICT: &[TailwindClass] = tw![
    "bg-[rgba(80,0,0,0.85)]",
    "border-[#ff4444]",
    "text-[#ff4444]",
];
states! {
    HotkeyBadgeState, Normal => NORMAL, Passive => PASSIVE, Conflict => CONFLICT
}
