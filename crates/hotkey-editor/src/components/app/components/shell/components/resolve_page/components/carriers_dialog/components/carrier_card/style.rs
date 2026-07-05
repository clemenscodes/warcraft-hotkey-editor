use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "flex",
    "flex-col",
    "items-center",
    "gap-[18px]",
    "m-0",
    "p-[30px_20px]",
    "bg-warcraft-bg-mid/50",
    "border",
    "border-warcraft-blue-bright-deep",
    "rounded-[8px]",
    "cursor-pointer",
    "hover:border-warcraft-gold",
    "hover:shadow-[0_0_8px_color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent)]",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
