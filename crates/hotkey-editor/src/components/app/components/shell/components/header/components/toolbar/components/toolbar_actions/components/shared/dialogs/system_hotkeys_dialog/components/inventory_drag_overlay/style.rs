use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "fixed",
    "pointer-events-none",
    "z-[1100]",
    "flex",
    "items-center",
    "justify-center",
    "border-solid",
    "border-[12px]",
    "select-none",
    "[background:linear-gradient(180deg,color-mix(in_oklab,var(--color-warcraft-bg-mid)_95%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-bg-base)_98%,transparent)_100%)]",
    "[border-image-source:var(--wc3-slot-frame)]",
    "[border-image-slice:12_fill]",
    "[border-image-repeat:stretch]",
    "[filter:drop-shadow(0_8px_24px_color-mix(in_oklab,var(--color-warcraft-shadow)_60%,transparent))_drop-shadow(0_0_16px_color-mix(in_oklab,var(--color-warcraft-gold)_60%,transparent))]",
];

const MOBILE: &[TailwindClass] = tw!["mobile:border-[8px]"];
const TABLET: &[TailwindClass] = tw!["tablet:border-[8px]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
