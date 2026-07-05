use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "px-[2.2rem]",
    "py-[1.8rem]",
    "border",
    "border-warcraft-gold/45",
    "rounded-xl",
    "bg-[linear-gradient(180deg,color-mix(in_oklab,var(--color-warcraft-gold-dark)_45%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-shadow)_35%,transparent)_100%)]",
    "shadow-[inset_0_0_0_1px_color-mix(in_oklab,var(--color-warcraft-gold)_8%,transparent),0_0_18px_color-mix(in_oklab,var(--color-warcraft-gold)_12%,transparent)]",
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
