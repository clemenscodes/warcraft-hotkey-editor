use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex",
    "flex-col",
    "gap-9",
    "py-10",
    "px-11",
    "border",
    "border-warcraft-gold-border",
    "rounded-[14px]",
    "text-left",
    "text-warcraft-text-secondary",
    "bg-[linear-gradient(180deg,color-mix(in_oklab,var(--color-warcraft-gold-dark)_55%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-shadow)_55%,transparent)_100%)]",
    "cursor-pointer",
    "transition-[border-color,color,background,box-shadow]",
    "duration-150",
    "ease-[ease]",
    "hover:border-warcraft-gold",
    "hover:text-warcraft-gold",
    "hover:bg-[linear-gradient(180deg,color-mix(in_oklab,var(--color-warcraft-gold)_18%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-gold-dark)_55%,transparent)_100%)]",
    "hover:shadow-[0_0_14px_color-mix(in_oklab,var(--color-warcraft-gold)_35%,transparent)]",
    "focus:outline-none",
    "kb-focus:outline-none",
    "kb-focus:border-warcraft-highlight",
    "kb-focus:text-warcraft-highlight",
    "kb-focus:shadow-[0_0_0_3px_var(--color-warcraft-highlight),0_0_16px_color-mix(in_oklab,var(--color-warcraft-highlight)_55%,transparent)]",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:gap-[10px]",
    "mobile:py-[12px]",
    "mobile:px-[14px]",
    "mobile:rounded-[10px]",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:gap-[10px]",
    "tablet:py-[12px]",
    "tablet:px-[14px]",
    "tablet:rounded-[10px]",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
