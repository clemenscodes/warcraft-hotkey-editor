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
    "bg-panel-gold-diag-85",
    "shadow-glow-22",
    "transition-[background,box-shadow]",
    "duration-[120ms]",
    "focus:outline-none",
    "focus-visible:border-white",
    "focus-visible:text-white",
    "focus-visible:shadow-ring-hl-3",
    "hover:bg-panel-gold-diag-22",
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
