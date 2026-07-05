use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "px-[3rem]",
    "py-[1.4rem]",
    "border",
    "border-warcraft-gold-border",
    "rounded-[10px]",
    "text-[2rem]",
    "tracking-[0.06em]",
    "uppercase",
    "text-warcraft-gold",
    "cursor-pointer",
    "[background:linear-gradient(180deg,color-mix(in_oklab,var(--color-warcraft-gold-dark)_65%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-shadow)_65%,transparent)_100%)]",
    "[text-shadow:1px_1px_0_var(--color-warcraft-shadow)]",
    "[transition:border-color_0.15s_ease,background_0.15s_ease,box-shadow_0.15s_ease]",
    "hover:border-warcraft-gold",
    "hover:[background:linear-gradient(180deg,color-mix(in_oklab,var(--color-warcraft-gold)_18%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-gold-dark)_65%,transparent)_100%)]",
    "hover:[box-shadow:0_0_10px_color-mix(in_oklab,var(--color-warcraft-gold)_35%,transparent)]",
    "active:translate-y-[1px]",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:w-full",
    "mobile:min-h-[44px]",
    "mobile:px-[24px]",
    "mobile:py-[12px]",
    "mobile:text-[16px]",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:w-full",
    "tablet:min-h-[44px]",
    "tablet:px-[24px]",
    "tablet:py-[12px]",
    "tablet:text-[16px]",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
