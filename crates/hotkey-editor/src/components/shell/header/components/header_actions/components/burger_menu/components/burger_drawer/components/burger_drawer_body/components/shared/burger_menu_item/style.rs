use super::state::BurgerItemState;
use crate::{classes, states};

const BASE: &[&str] = &[
    "flex",
    "items-center",
    "gap-[0.85rem]",
    "w-full",
    "min-h-12",
    "py-[0.65rem]",
    "px-[0.9rem]",
    "[background:linear-gradient(180deg,rgba(40,30,8,0.55)_0%,rgba(15,12,4,0.55)_100%)]",
    "border",
    "border-[#6c5a1f]",
    "rounded-[10px]",
    "text-warcraft-text-secondary",
    "font-friz-quadrata",
    "text-[1rem]",
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

const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}

const IDLE: &[&str] = &[];

const ACTIVE: &[&str] = &[
    "[background:linear-gradient(180deg,rgba(255,206,99,0.22)_0%,rgba(40,30,8,0.6)_100%)]",
    "border-warcraft-gold",
    "text-warcraft-gold",
    "[box-shadow:inset_0_0_0_1px_rgba(255,206,99,0.3),0_0_14px_rgba(255,206,99,0.22)]",
];

const PRIMARY: &[&str] = &[
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
