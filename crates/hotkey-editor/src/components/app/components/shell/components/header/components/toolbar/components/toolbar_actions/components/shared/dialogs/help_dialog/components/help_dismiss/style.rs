use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "inline-flex",
    "items-center",
    "justify-center",
    "min-h-[3rem]",
    "px-[1.8rem]",
    "py-[0.7rem]",
    "border",
    "border-warcraft-gold",
    "rounded-[10px]",
    "text-warcraft-gold",
    "text-[1.4rem]",
    "tracking-[0.08em]",
    "uppercase",
    "cursor-pointer",
    "bg-[linear-gradient(135deg,color-mix(in_oklab,var(--color-warcraft-gold-dark)_85%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-shadow)_85%,transparent)_100%)]",
    "shadow-[0_0_22px_color-mix(in_oklab,var(--color-warcraft-gold)_22%,transparent)]",
    "transition-[background,box-shadow]",
    "duration-[120ms]",
    "focus:outline-none",
    "focus-visible:border-white",
    "focus-visible:text-white",
    "focus-visible:shadow-[0_0_0_3px_var(--color-warcraft-highlight),0_0_18px_color-mix(in_oklab,var(--color-warcraft-highlight)_55%,transparent)]",
    "hover:bg-[linear-gradient(135deg,color-mix(in_oklab,var(--color-warcraft-gold)_22%,transparent)_0%,color-mix(in_oklab,var(--color-race-neutral-strong)_95%,transparent)_100%)]",
    "hover:shadow-[0_0_26px_color-mix(in_oklab,var(--color-warcraft-gold)_55%,transparent)]",
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
