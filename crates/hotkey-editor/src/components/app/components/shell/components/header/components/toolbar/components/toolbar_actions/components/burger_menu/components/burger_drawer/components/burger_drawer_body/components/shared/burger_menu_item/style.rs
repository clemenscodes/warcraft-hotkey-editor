use super::state::BurgerItemState;
use crate::{classes, states, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex",
    "items-center",
    "gap-[0.7rem]",
    "w-full",
    "min-h-10",
    "py-[0.5rem]",
    "px-[0.8rem]",
    "[background:linear-gradient(180deg,rgba(40,30,8,0.55)_0%,rgba(15,12,4,0.55)_100%)]",
    "border",
    "border-[#6c5a1f]",
    "rounded-[8px]",
    "text-warcraft-text-secondary",
    "text-[0.9rem]",
    "tracking-[0.05em]",
    "text-left",
    "cursor-pointer",
    "[transition:border-color_0.15s_ease,color_0.15s_ease,background_0.15s_ease,box-shadow_0.15s_ease]",
    "hover:border-warcraft-gold",
    "hover:text-warcraft-gold",
    "hover:[background:linear-gradient(180deg,rgba(255,206,99,0.18)_0%,rgba(40,30,8,0.55)_100%)]",
    "hover:[box-shadow:0_0_12px_rgba(255,206,99,0.3)]",
    "focus:outline-none",
    "focus-visible:border-white",
    "focus-visible:text-white",
    "focus-visible:[box-shadow:0_0_0_2px_#fff,0_0_16px_rgba(255,255,255,0.55)]",
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

const IDLE: &[TailwindClass] = tw![];

const ACTIVE: &[TailwindClass] = tw![
    "[background:linear-gradient(180deg,rgba(255,206,99,0.22)_0%,rgba(40,30,8,0.6)_100%)]",
    "border-warcraft-gold",
    "text-warcraft-gold",
    "[box-shadow:inset_0_0_0_1px_rgba(255,206,99,0.3),0_0_14px_rgba(255,206,99,0.22)]",
];

const PRIMARY: &[TailwindClass] = tw![
    "[background:linear-gradient(135deg,rgba(40,30,8,0.85)_0%,rgba(15,12,4,0.85)_100%)]",
    "border-warcraft-gold",
    "text-warcraft-gold",
    "[box-shadow:0_0_22px_rgba(255,206,99,0.22)]",
    "hover:[background:linear-gradient(135deg,rgba(255,206,99,0.22)_0%,rgba(60,45,14,0.95)_100%)]",
    "hover:[box-shadow:0_0_26px_rgba(255,206,99,0.55),inset_0_0_14px_rgba(255,206,99,0.15)]",
];
states! {
    BurgerItemState, Idle => IDLE, Active => ACTIVE, Primary => PRIMARY
}
