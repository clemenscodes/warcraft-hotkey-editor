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
    "bg-panel-gold-resting",
    "border",
    "border-warcraft-gold-border",
    "rounded-[8px]",
    "text-warcraft-text-secondary",
    "text-[0.9rem]",
    "tracking-[0.05em]",
    "text-left",
    "cursor-pointer",
    "[transition:border-color_0.15s_ease,color_0.15s_ease,background_0.15s_ease,box-shadow_0.15s_ease]",
    "hover:border-warcraft-gold",
    "hover:text-warcraft-gold",
    "hover:bg-panel-gold-active",
    "hover:shadow-glow-12",
    "focus:outline-none",
    "focus-visible:border-white",
    "focus-visible:text-white",
    "focus-visible:shadow-ring-hl",
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
    "[background:linear-gradient(180deg,color-mix(in_oklab,var(--color-warcraft-gold)_22%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-gold-dark)_60%,transparent)_100%)]",
    "border-warcraft-gold",
    "text-warcraft-gold",
    "[box-shadow:inset_0_0_0_1px_color-mix(in_oklab,var(--color-warcraft-gold)_30%,transparent),0_0_14px_color-mix(in_oklab,var(--color-warcraft-gold)_22%,transparent)]",
];

const PRIMARY: &[TailwindClass] = tw![
    "bg-panel-gold-diag-85",
    "border-warcraft-gold",
    "text-warcraft-gold",
    "shadow-glow-22",
    "hover:bg-panel-gold-diag-22",
    "hover:[box-shadow:0_0_26px_color-mix(in_oklab,var(--color-warcraft-gold)_55%,transparent),inset_0_0_14px_color-mix(in_oklab,var(--color-warcraft-gold)_15%,transparent)]",
];
states! {
    BurgerItemState, Idle => IDLE, Active => ACTIVE, Primary => PRIMARY
}
